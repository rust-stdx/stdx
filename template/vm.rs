//! Tree-walking interpreter that evaluates template AST nodes and expressions.

use alloc::{
    collections::BTreeMap,
    format,
    rc::Rc,
    string::{String, ToString},
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

pub struct Renderer<'a> {
    pub engine: &'a crate::engine::Engine,
    pub out: &'a mut dyn fmt::Write,
    pub scopes: Vec<BTreeMap<String, Value>>,
    pub escaper: EscaperFn,
    pub block_overrides: BTreeMap<String, Vec<NodeList>>,
    pub include_depth: usize,
}

const MAX_INCLUDE_DEPTH: usize = 64;

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
            crate::engine::EscapeMode::Text => |s: &str, out: &mut String| {
                out.push_str(s);
                Ok(())
            },
        };

        Self {
            engine,
            out,
            scopes: vec![top_scope],
            escaper,
            block_overrides: BTreeMap::new(),
            include_depth: 0,
        }
    }

    pub fn render_nodes(&mut self, nodes: &[Node]) -> Result<(), Error> {
        for node in nodes {
            self.render_node(node)?;
        }
        Ok(())
    }

    fn render_node(&mut self, node: &Node) -> Result<(), Error> {
        match node {
            Node::Raw(s) => {
                self.out.write_str(s).map_err(|e| Error::render(e.to_string()))?;
            }
            Node::Expr(expr) => {
                let val = self.eval_expr(expr)?;
                match &val {
                    Value::Safe(s) => {
                        self.out.write_str(s).map_err(|e| Error::render(e.to_string()))?;
                    }
                    Value::Str(s) => {
                        let mut escaped = String::new();
                        (self.escaper)(s, &mut escaped)?;
                        self.out.write_str(&escaped).map_err(|e| Error::render(e.to_string()))?;
                    }
                    _ => {
                        let mut s = String::new();
                        val.fmt_to(&mut s).map_err(|e| Error::render(e.to_string()))?;
                        let mut escaped = String::new();
                        (self.escaper)(&s, &mut escaped)?;
                        self.out.write_str(&escaped).map_err(|e| Error::render(e.to_string()))?;
                    }
                }
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
                self.include_depth += 1;
                self.render_nodes(&template.nodes)?;
                self.include_depth -= 1;
            }
            Node::Extends(_name) => {
                // Extends is handled at a higher level (in render_template)
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
                    self.render_nodes(&chain[0])?;
                    self.scopes.pop();
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
                self.out.write_str(s).map_err(|e| Error::render(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn render_if(&mut self, if_node: &IfNode) -> Result<(), Error> {
        let cond = self.eval_expr(&if_node.condition)?;
        if cond.is_truthy() {
            self.render_nodes(&if_node.body)?;
            return Ok(());
        }
        for elif in &if_node.elifs {
            let elif_cond = self.eval_expr(&elif.condition)?;
            if elif_cond.is_truthy() {
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
        let items: Vec<Value> = match &iterable {
            Value::Array(arr) => arr.iter().cloned().collect(),
            Value::Str(s) => s
                .chars()
                .map(|c| {
                    if c.is_ascii() {
                        let mut buf = [0u8; 4];
                        Value::Str(c.encode_utf8(&mut buf).into())
                    } else {
                        Value::Str(c.to_string().into())
                    }
                })
                .collect(),
            _ => Vec::new(),
        };
        for item in items {
            let mut scope = BTreeMap::new();
            scope.insert(for_node.var_name.clone(), item);
            self.scopes.push(scope);
            self.render_nodes(&for_node.body)?;
            self.scopes.pop();
        }
        Ok(())
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Var(name) => {
                if name == "super" {
                    if let Some(scope) = self.scopes.last() {
                        if let Some(Value::Str(s)) = scope.get("__parent__") {
                            return Ok(Value::Safe(Rc::clone(s)));
                        }
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
                        // .fract() is not available in `no_std` environments, so we
                        // cast through i64 to check that there is not fractional part.
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
                // Short-circuit evaluation for And / Or
                let r = match op {
                    BinOp::And if !l.is_truthy() => return Ok(Value::Bool(false)),
                    BinOp::Or if l.is_truthy() => return Ok(Value::Bool(true)),
                    _ => self.eval_expr(right)?,
                };
                self.eval_binop(&l, op, &r)
            }
            Expr::UnaryOp {
                op,
                expr: inner,
            } => {
                let val = self.eval_expr(inner)?;
                match op {
                    UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                    UnaryOp::Neg => match val {
                        Value::I64(n) => Ok(Value::I64(-n)),
                        Value::F64(n) => Ok(Value::F64(-n)),
                        _ => Ok(Value::Null),
                    },
                }
            }
            Expr::Call(name, args) => {
                let eval_args: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect::<Result<Vec<_>, _>>()?;
                match name.as_str() {
                    "super" => {
                        if let Some(scope) = self.scopes.last() {
                            if let Some(Value::Str(v)) = scope.get("__parent__") {
                                return Ok(Value::Safe(Rc::clone(v)));
                            }
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
                        let end = range_args.get(1);
                        match end {
                            Some(end) => {
                                let items: Vec<Value> = (start..*end).map(Value::I64).collect();
                                Ok(Value::Array(Rc::new(items)))
                            }
                            None => {
                                let items: Vec<Value> = (0..start).map(Value::I64).collect();
                                Ok(Value::Array(Rc::new(items)))
                            }
                        }
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
            BinOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            BinOp::Add => self.arithmetic_op(left, right, |a, b| a + b, |a, b| a + b),
            BinOp::Sub => self.arithmetic_op(left, right, |a, b| a - b, |a, b| a - b),
            BinOp::Mul => self.arithmetic_op(left, right, |a, b| a * b, |a, b| a * b),
            BinOp::Div => {
                if matches!((left, right), (Value::I64(_), Value::I64(0))) {
                    return Err(Error::render("division by zero"));
                }
                self.arithmetic_op(left, right, |a, b| a / b, |a, b| a / b)
            }
            BinOp::Mod => {
                if matches!((left, right), (Value::I64(_), Value::I64(0))) {
                    return Err(Error::render("modulo by zero"));
                }
                self.arithmetic_op(left, right, |a, b| a % b, |a, b| a % b)
            }
            BinOp::In => {
                let found = match right {
                    Value::Array(arr) => arr.iter().any(|v| v == left),
                    Value::Str(s) => {
                        let needle = match left {
                            Value::Str(n) => Some(n.as_ref()),
                            _ => None,
                        };
                        needle.map(|n| s.contains(n)).unwrap_or(false)
                    }
                    _ => false,
                };
                Ok(Value::Bool(found))
            }
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
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(cmp(a.cmp(b)))),
            _ => Ok(Value::Bool(false)),
        }
    }

    fn arithmetic_op(
        &self,
        left: &Value,
        right: &Value,
        i_op: fn(i64, i64) -> i64,
        f_op: fn(f64, f64) -> f64,
    ) -> Result<Value, Error> {
        match (left, right) {
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(i_op(*a, *b))),
            (Value::I64(a), Value::F64(b)) => Ok(Value::F64(f_op(*a as f64, *b))),
            (Value::F64(a), Value::I64(b)) => Ok(Value::F64(f_op(*a, *b as f64))),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(f_op(*a, *b))),
            _ => Ok(Value::Null),
        }
    }
}
