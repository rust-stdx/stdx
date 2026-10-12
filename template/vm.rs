//! Tree-walking interpreter that evaluates template AST nodes and expressions.

use alloc::{
    collections::BTreeMap,
    format,
    rc::Rc,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::fmt;

use crate::{
    ast::{BinOp, Expr, ForNode, IfNode, Node, NodeList, UnaryOp},
    error::Error,
    escapers::{EscaperFn, html_escape},
    filters::builtin_filters,
    value::Value,
};

const MAX_INCLUDE_DEPTH: usize = 64;

/// Maximum nesting of block nodes (`if` / `for` / `include`) during a render.
/// Exceeding it returns an error rather than overflowing the stack.
const MAX_RENDER_DEPTH: usize = 500;

/// Maximum nesting of expressions during evaluation (parentheses, binary
/// operators, filters, index/dot chains). Exceeding it returns an error rather
/// than overflowing the stack.
const MAX_EVAL_DEPTH: usize = 500;

/// Human-readable name of a value's type, for error messages.
fn describe(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::I64(_) => "an integer",
        Value::F64(_) => "a float",
        Value::Str(_) | Value::Safe(_) => "a string",
        Value::Array(_) => "an array",
        Value::Map(_) => "an object",
    }
}

pub struct Renderer<'a> {
    pub engine: &'a crate::engine::Engine,
    pub out: &'a mut dyn fmt::Write,
    pub scopes: Vec<BTreeMap<String, Value>>,
    pub escaper: EscaperFn,
    pub block_overrides: BTreeMap<String, Vec<Arc<NodeList>>>,
    pub include_depth: usize,
    scratch: String,
    render_depth: usize,
    eval_depth: usize,
}

impl<'a> Renderer<'a> {
    pub fn new(engine: &'a crate::engine::Engine, out: &'a mut dyn fmt::Write, variables: Value) -> Self {
        let mut top_scope = BTreeMap::new();
        if let Value::Map(m) = variables {
            for (k, v) in m.iter() {
                top_scope.insert(k.clone(), v.clone());
            }
        }

        let escaper = match engine.mode {
            crate::engine::EscapeMode::Html => html_escape,
            crate::engine::EscapeMode::Text => |s: &str, out: &mut dyn fmt::Write| out.write_str(s),
        };

        Self {
            engine,
            out,
            scopes: vec![top_scope],
            escaper,
            block_overrides: BTreeMap::new(),
            include_depth: 0,
            scratch: String::new(),
            render_depth: 0,
            eval_depth: 0,
        }
    }

    pub fn render_nodes(&mut self, nodes: &[Node]) -> Result<(), Error> {
        self.render_depth += 1;
        if self.render_depth > MAX_RENDER_DEPTH {
            self.render_depth -= 1;
            return Err(Error::render(format!("render nesting exceeds the limit of {MAX_RENDER_DEPTH}")));
        }
        for node in nodes {
            if let Err(e) = self.render_node(node) {
                self.render_depth -= 1;
                return Err(e);
            }
        }
        self.render_depth -= 1;
        Ok(())
    }

    fn render_node(&mut self, node: &Node) -> Result<(), Error> {
        match node {
            Node::Raw(s) => {
                self.out.write_str(s).map_err(|_| write_error())?;
            }
            Node::Expr(expr) => {
                let val = self.eval_expr(expr)?;
                self.write_value(&val)?;
            }
            Node::If(if_node) => {
                self.render_if(if_node)?;
            }
            Node::For(for_node) => {
                self.render_for(for_node)?;
            }
            Node::Include(name) => {
                if self.include_depth >= MAX_INCLUDE_DEPTH {
                    return Err(Error::render(format!("include depth limit ({MAX_INCLUDE_DEPTH}) exceeded")));
                }
                let template = self
                    .engine
                    .templates
                    .get(name.as_str())
                    .ok_or_else(|| Error::undefined_template(name.clone()))?;
                if template.nodes.iter().any(|n| matches!(n, Node::Extends(_))) {
                    return Err(Error::render(format!(
                        "included template `{name}` uses `{{% extends %}}`, which is not supported inside `{{% include %}}`"
                    )));
                }
                self.include_depth += 1;
                self.render_nodes(&template.nodes)?;
                self.include_depth -= 1;
            }
            Node::Extends(_name) => {
                // Extends is handled at a higher level (in render_with_extends)
                // If we encounter it here, it's effectively a no-op
            }
            Node::Block(block) => {
                let chain = self.block_overrides.get(&block.name).cloned();
                if let Some(chain) = chain {
                    let mut output = String::new();
                    for parent_body in chain[1..].iter().rev() {
                        let mut buf = String::new();
                        {
                            let mut sub = Renderer {
                                engine: self.engine,
                                out: &mut buf,
                                scopes: self.scopes.clone(),
                                escaper: self.escaper,
                                block_overrides: self.block_overrides.clone(),
                                include_depth: self.include_depth,
                                scratch: String::new(),
                                render_depth: 0,
                                eval_depth: 0,
                            };
                            if !output.is_empty() {
                                let mut scope = BTreeMap::new();
                                scope.insert("__parent__".into(), Value::Str(output.clone().into()));
                                sub.scopes.push(scope);
                            }
                            sub.render_nodes(parent_body)?;
                        }
                        output = buf;
                    }
                    let mut scope = BTreeMap::new();
                    scope.insert("__parent__".into(), Value::Str(output.clone().into()));
                    self.scopes.push(scope);
                    let result = self.render_nodes(&chain[0]);
                    self.scopes.pop();
                    result?;
                } else {
                    self.render_nodes(&block.body)?;
                }
            }
            Node::Set(name, expr) => {
                let val = self.eval_expr(expr)?;
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.clone(), val);
                }
            }
            Node::RawBlock(s) => {
                self.out.write_str(s).map_err(|_| write_error())?;
            }
        }
        Ok(())
    }

    /// Write a value to the output, escaping it unless it is already [`Value::Safe`].
    fn write_value(&mut self, val: &Value) -> Result<(), Error> {
        match val {
            Value::Safe(s) => {
                self.out.write_str(s).map_err(|_| write_error())?;
            }
            Value::Str(s) => {
                let escaper = self.escaper;
                escaper(s, self.out).map_err(|_| write_error())?;
            }
            other => {
                self.scratch.clear();
                other.fmt_to(&mut self.scratch).map_err(|_| write_error())?;
                let escaper = self.escaper;
                escaper(&self.scratch, self.out).map_err(|_| write_error())?;
            }
        }
        Ok(())
    }

    fn expect_bool(&self, v: &Value, context: &str) -> Result<bool, Error> {
        match v {
            Value::Bool(b) => Ok(*b),
            other => Err(Error::r#type(format!(
                "expected a boolean in {context}, got {}",
                describe(other)
            ))),
        }
    }

    fn render_if(&mut self, if_node: &IfNode) -> Result<(), Error> {
        let cond = self.eval_expr(&if_node.condition)?;
        if self.expect_bool(&cond, "`if` condition")? {
            self.render_nodes(&if_node.body)?;
            return Ok(());
        }
        for elif in &if_node.elifs {
            let elif_cond = self.eval_expr(&elif.condition)?;
            if self.expect_bool(&elif_cond, "`elif` condition")? {
                self.render_nodes(&elif.body)?;
                return Ok(());
            }
        }
        if let Some(else_body) = &if_node.else_body {
            self.render_nodes(else_body)?;
        }
        Ok(())
    }

    fn render_for(&mut self, for_node: &ForNode) -> Result<(), Error> {
        let iterable = self.eval_expr(&for_node.iterable)?;
        // Push a single scope frame reused by every iteration instead of
        // allocating a fresh map per item.
        let mut scope = BTreeMap::new();
        scope.insert(for_node.var_name.clone(), Value::Null);
        self.scopes.push(scope);
        let result = self.render_for_items(&iterable, &for_node.var_name, &for_node.body);
        self.scopes.pop();
        result
    }

    fn render_for_items(&mut self, iterable: &Value, var: &str, body: &[Node]) -> Result<(), Error> {
        let assign = |scope: &mut BTreeMap<String, Value>, item: Value| {
            if let Some(slot) = scope.get_mut(var) {
                *slot = item;
            }
        };
        match iterable {
            Value::Array(arr) => {
                let arr = Rc::clone(arr);
                for item in arr.iter() {
                    assign(self.scopes.last_mut().expect("loop scope"), item.clone());
                    self.render_nodes(body)?;
                }
            }
            Value::Str(s) => {
                for c in s.chars() {
                    assign(self.scopes.last_mut().expect("loop scope"), Value::Str(c.to_string().into()));
                    self.render_nodes(body)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        self.eval_depth += 1;
        if self.eval_depth > MAX_EVAL_DEPTH {
            self.eval_depth -= 1;
            return Err(Error::render(format!(
                "expression nesting exceeds the limit of {MAX_EVAL_DEPTH}"
            )));
        }
        let result = self.eval_expr_inner(expr);
        self.eval_depth -= 1;
        result
    }

    fn eval_expr_inner(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Var(name) => {
                if name == "super" {
                    if let Some(scope) = self.scopes.last()
                        && let Some(Value::Str(s) | Value::Safe(s)) = scope.get("__parent__")
                    {
                        return Ok(Value::Safe(Rc::clone(s)));
                    }
                    return Ok(Value::Null);
                }
                for scope in self.scopes.iter().rev() {
                    if let Some(v) = scope.get(name.as_str()) {
                        return Ok(v.clone());
                    }
                }
                Ok(Value::Null)
            }
            Expr::Dot(inner, field) => {
                let val = self.eval_expr(inner)?;
                Ok(val.get(field).unwrap_or(Value::Null))
            }
            Expr::Index(inner, index_expr) => {
                let val = self.eval_expr(inner)?;
                let idx_val = self.eval_expr(index_expr)?;
                match idx_val {
                    Value::I64(n) => {
                        if n < 0 {
                            return Err(Error::render("negative index is not allowed"));
                        }
                        Ok(val.get_index(n as usize).unwrap_or(Value::Null))
                    }
                    Value::F64(n) => {
                        // `.fract()` is not available in `no_std` environments, so we
                        // cast through i64 to check that there is no fractional part.
                        // Rejects floats with decimals or outside i64 range.
                        if n < 0.0 || !n.is_finite() || (n as i64) as f64 != n {
                            return Err(Error::render("invalid index"));
                        }
                        Ok(val.get_index(n as usize).unwrap_or(Value::Null))
                    }
                    Value::Str(s) => Ok(val.get(&s).unwrap_or(Value::Null)),
                    _ => Ok(Value::Null),
                }
            }
            Expr::Str(s) => Ok(Value::Str(s.as_str().into())),
            Expr::I64(n) => Ok(Value::I64(*n)),
            Expr::F64(n) => Ok(Value::F64(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Filter {
                expr: inner,
                name,
                args,
            } => {
                let val = self.eval_expr(inner)?;
                let eval_args: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect::<Result<Vec<_>, _>>()?;
                let filters = builtin_filters();
                let filter_fn = filters
                    .binary_search_by_key(&name.as_str(), |&(k, _)| k)
                    .ok()
                    .map(|i| filters[i].1)
                    .ok_or_else(|| Error::undefined_filter(name.clone(), 0, 0))?;
                filter_fn(&val, &eval_args)
            }
            Expr::BinOp {
                left,
                op,
                right,
            } => {
                let l = self.eval_expr(left)?;
                match op {
                    BinOp::And => {
                        if !self.expect_bool(&l, "`and`")? {
                            return Ok(Value::Bool(false));
                        }
                        self.eval_expr(right)
                    }
                    BinOp::Or => {
                        if self.expect_bool(&l, "`or`")? {
                            return Ok(Value::Bool(true));
                        }
                        self.eval_expr(right)
                    }
                    _ => {
                        let r = self.eval_expr(right)?;
                        self.eval_binop(&l, op, &r)
                    }
                }
            }
            Expr::UnaryOp {
                op,
                expr: inner,
            } => {
                let val = self.eval_expr(inner)?;
                match op {
                    UnaryOp::Not => Ok(Value::Bool(!self.expect_bool(&val, "`not`")?)),
                    UnaryOp::Neg => match val {
                        Value::I64(n) => n
                            .checked_neg()
                            .map(Value::I64)
                            .ok_or_else(|| Error::render("integer overflow in negation")),
                        Value::F64(n) => Ok(Value::F64(-n)),
                        _ => Ok(Value::Null),
                    },
                }
            }
            // `is_set` is evaluated lazily so that a missing nested field (e.g.
            // `is_set(a.b.c)`) yields `false` instead of an error.
            Expr::Call(name, args) if name == "is_set" => {
                if args.len() != 1 {
                    return Err(Error::render(format!(
                        "is_set() takes exactly one argument, got {}",
                        args.len()
                    )));
                }
                match self.eval_expr(&args[0]) {
                    Ok(v) => Ok(Value::Bool(!matches!(v, Value::Null))),
                    Err(_) => Ok(Value::Bool(false)),
                }
            }
            Expr::Call(name, args) => {
                let eval_args: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect::<Result<Vec<_>, _>>()?;
                match name.as_str() {
                    "super" => {
                        if let Some(scope) = self.scopes.last()
                            && let Some(Value::Str(v) | Value::Safe(v)) = scope.get("__parent__")
                        {
                            return Ok(Value::Safe(Rc::clone(v)));
                        }
                        Ok(Value::Null)
                    }
                    "range" => {
                        if eval_args.is_empty() {
                            return Err(Error::render("range() requires at least one argument"));
                        }
                        if eval_args.len() > 2 {
                            return Err(Error::render("range() takes at most two arguments"));
                        }
                        let mut range_args: Vec<i64> = Vec::with_capacity(eval_args.len());
                        for (i, arg) in eval_args.iter().enumerate() {
                            match arg {
                                Value::I64(n) => range_args.push(*n),
                                _ => {
                                    return Err(Error::render(format!(
                                        "range() argument {} must be an integer",
                                        i + 1
                                    )));
                                }
                            }
                        }
                        let start = *range_args.first().unwrap_or(&0);
                        let (from, to) = match range_args.get(1) {
                            Some(end) => (start, *end),
                            None => (0, start),
                        };
                        // Compute the size without overflowing and reject ranges
                        // that would allocate too much memory.
                        let count = to as i128 - from as i128;
                        let limit = self.engine.range_limit;
                        if count > limit as i128 {
                            return Err(Error::render(format!(
                                "range() would produce {count} items, exceeding the limit of {limit}"
                            )));
                        }
                        if count <= 0 {
                            return Ok(Value::Array(Rc::new(Vec::new())));
                        }
                        let items: Vec<Value> = (from..to).map(Value::I64).collect();
                        Ok(Value::Array(Rc::new(items)))
                    }
                    _ => Err(Error::render(format!("unknown function `{name}`"))),
                }
            }
        }
    }

    fn eval_binop(&self, left: &Value, op: &BinOp, right: &Value) -> Result<Value, Error> {
        match op {
            BinOp::Eq => Ok(Value::Bool(match (left, right) {
                (Value::F64(a), Value::F64(b)) if a.is_nan() || b.is_nan() => false,
                _ => left == right,
            })),
            BinOp::Neq => Ok(Value::Bool(match (left, right) {
                (Value::F64(a), Value::F64(b)) if a.is_nan() || b.is_nan() => true,
                _ => left != right,
            })),
            BinOp::Lt => self.cmp_op(left, right, |o| o == core::cmp::Ordering::Less),
            BinOp::Gt => self.cmp_op(left, right, |o| o == core::cmp::Ordering::Greater),
            BinOp::Lte => self.cmp_op(left, right, |o| o != core::cmp::Ordering::Greater),
            BinOp::Gte => self.cmp_op(left, right, |o| o != core::cmp::Ordering::Less),
            BinOp::Add => self.arithmetic_op(left, right, i64::checked_add, |a, b| a + b, "addition"),
            BinOp::Sub => self.arithmetic_op(left, right, i64::checked_sub, |a, b| a - b, "subtraction"),
            BinOp::Mul => self.arithmetic_op(left, right, i64::checked_mul, |a, b| a * b, "multiplication"),
            BinOp::Div => {
                if matches!((left, right), (Value::I64(_), Value::I64(0))) {
                    return Err(Error::render("division by zero"));
                }
                self.arithmetic_op(left, right, i64::checked_div, |a, b| a / b, "division")
            }
            BinOp::Mod => {
                if matches!((left, right), (Value::I64(_), Value::I64(0))) {
                    return Err(Error::render("modulo by zero"));
                }
                self.arithmetic_op(left, right, i64::checked_rem, |a, b| a % b, "modulo")
            }
            BinOp::In => {
                let found = match right {
                    Value::Array(arr) => arr.iter().any(|v| v == left),
                    Value::Str(s) | Value::Safe(s) => {
                        let needle = match left {
                            Value::Str(n) | Value::Safe(n) => Some(n.as_ref()),
                            _ => None,
                        };
                        needle.map(|n| s.contains(n)).unwrap_or(false)
                    }
                    _ => false,
                };
                Ok(Value::Bool(found))
            }
            // `and` / `or` are handled (with short-circuiting) in `eval_expr`.
            BinOp::And | BinOp::Or => unreachable!("logical operators are handled in eval_expr"),
        }
    }

    fn cmp_op<F>(&self, left: &Value, right: &Value, cmp: F) -> Result<Value, Error>
    where
        F: FnOnce(core::cmp::Ordering) -> bool,
    {
        match (left, right) {
            (Value::I64(a), Value::I64(b)) => Ok(Value::Bool(cmp(a.cmp(b)))),
            (Value::I64(a), Value::F64(b)) => {
                let a = *a as f64;
                if a.is_nan() || b.is_nan() {
                    return Ok(Value::Bool(false));
                }
                Ok(Value::Bool(cmp(a.total_cmp(b))))
            }
            (Value::F64(a), Value::I64(b)) => {
                let b = *b as f64;
                if a.is_nan() || b.is_nan() {
                    return Ok(Value::Bool(false));
                }
                Ok(Value::Bool(cmp(a.total_cmp(&b))))
            }
            (Value::F64(a), Value::F64(b)) => {
                if a.is_nan() || b.is_nan() {
                    return Ok(Value::Bool(false));
                }
                Ok(Value::Bool(cmp(a.total_cmp(b))))
            }
            (Value::Str(a) | Value::Safe(a), Value::Str(b) | Value::Safe(b)) => Ok(Value::Bool(cmp(a.cmp(b)))),
            _ => Ok(Value::Bool(false)),
        }
    }

    fn arithmetic_op(
        &self,
        left: &Value,
        right: &Value,
        i_op: fn(i64, i64) -> Option<i64>,
        f_op: fn(f64, f64) -> f64,
        op_name: &str,
    ) -> Result<Value, Error> {
        match (left, right) {
            (Value::I64(a), Value::I64(b)) => i_op(*a, *b)
                .map(Value::I64)
                .ok_or_else(|| Error::render(format!("integer overflow in {op_name}"))),
            (Value::I64(a), Value::F64(b)) => Ok(Value::F64(f_op(*a as f64, *b))),
            (Value::F64(a), Value::I64(b)) => Ok(Value::F64(f_op(*a, *b as f64))),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(f_op(*a, *b))),
            _ => Ok(Value::Null),
        }
    }
}

fn write_error() -> Error {
    Error::render("failed to write output")
}
