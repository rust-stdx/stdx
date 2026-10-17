//! Expression lexer and recursive-descent parser for template expressions.

use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::{
    ast::{BinOp, Expr, UnaryOp},
    error::{Error, SourcePosition},
};

#[derive(Clone, Debug)]
pub enum Token {
    Ident(String),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    Dot,
    Pipe,
    Comma,
    Colon,
    Equals,
    LParen,
    RParen,
    LBracket,
    RBracket,
    OpEq,
    OpNeq,
    OpLt,
    OpGt,
    OpLte,
    OpGte,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    KeywordAnd,
    KeywordOr,
    KeywordNot,
    KeywordIn,
}

pub struct ExprParser {
    tokens: Vec<(Token, SourcePosition)>,
    pos: usize,
    depth: usize,
}

/// Maximum nesting depth of an expression (parentheses, `not`, index, call and
/// filter arguments). Exceeding it returns an error rather than overflowing the
/// stack. Kept conservative because each level costs several parser frames.
const MAX_EXPR_DEPTH: usize = 96;

/// Maximum number of tokens in a single expression.
///
/// Bounds the size (and therefore the depth) of the resulting AST so that
/// validating or dropping a pathological expression cannot overflow the stack.
const MAX_EXPR_TOKENS: usize = 4096;

impl ExprParser {
    pub fn new(tokens: Vec<(Token, SourcePosition)>) -> Self {
        Self {
            tokens,
            pos: 0,
            depth: 0,
        }
    }

    pub fn parse_all(mut self) -> Result<Expr, Error> {
        if self.tokens.len() > MAX_EXPR_TOKENS {
            return Err(Error::parse(format!(
                "expression is too large ({} tokens, limit {MAX_EXPR_TOKENS})",
                self.tokens.len()
            )));
        }
        let expr = self.parse_or()?;
        if self.pos < self.tokens.len() {
            let (_, pos) = &self.tokens[self.pos];
            return Err(Error::syntax("unexpected trailing tokens", pos.line, pos.column));
        }
        validate_types(&expr)?;
        Ok(expr)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _)| t)
    }

    fn peek_position(&self) -> SourcePosition {
        self.tokens
            .get(self.pos)
            .map(|(_, p)| p.clone())
            .unwrap_or(SourcePosition::new(0, 0))
    }

    /// Position of the most recently consumed token (or `0:0` for an empty
    /// token stream).
    fn last_position(&self) -> SourcePosition {
        self.tokens
            .last()
            .map(|(_, p)| p.clone())
            .unwrap_or(SourcePosition::new(0, 0))
    }

    fn advance(&mut self) -> Option<&(Token, SourcePosition)> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect(&mut self) -> Result<Token, Error> {
        self.advance()
            .map(|(t, _)| t.clone())
            .ok_or_else(|| Error::parse("unexpected end of expression"))
    }

    fn expect_position(&mut self, msg: &str) -> Result<(Token, SourcePosition), Error> {
        let pos = self.last_position();
        self.advance()
            .cloned()
            .ok_or_else(|| Error::syntax(msg, pos.line, pos.column))
    }

    fn expect_ident(&mut self) -> Result<String, Error> {
        match self.expect()? {
            Token::Ident(s) => Ok(s),
            tok => Err(Error::parse(format!("expected identifier, got {tok:?}"))),
        }
    }

    fn parse_or(&mut self) -> Result<Expr, Error> {
        self.depth += 1;
        if self.depth > MAX_EXPR_DEPTH {
            self.depth -= 1;
            return Err(Error::r#type(format!(
                "expression nesting exceeds the limit of {MAX_EXPR_DEPTH}"
            )));
        }
        let result = self.parse_or_inner();
        self.depth -= 1;
        result
    }

    fn parse_or_inner(&mut self) -> Result<Expr, Error> {
        // Collect operands iteratively and fold right-associatively, so that
        // `a or b or c` evaluates as `a or (b or c)`: every operand except the
        // last is a boolean decision, and the last is the fallback value.
        let mut operands = vec![self.parse_and()?];
        while matches!(self.peek_token(), Some(Token::KeywordOr)) {
            self.advance();
            operands.push(self.parse_and()?);
        }
        let mut expr = operands.pop().expect("at least one operand");
        while let Some(left) = operands.pop() {
            expr = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(expr),
            };
        }
        self.parse_filter_pipes(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, Error> {
        let mut operands = vec![self.parse_comparison()?];
        while matches!(self.peek_token(), Some(Token::KeywordAnd)) {
            self.advance();
            operands.push(self.parse_comparison()?);
        }
        let mut expr = operands.pop().expect("at least one operand");
        while let Some(left) = operands.pop() {
            expr = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(expr),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Error> {
        let left = self.parse_addition()?;
        match self.peek_token() {
            Some(Token::OpEq) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Eq,
                    right: Box::new(right),
                })
            }
            Some(Token::OpNeq) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Neq,
                    right: Box::new(right),
                })
            }
            Some(Token::OpLt) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Lt,
                    right: Box::new(right),
                })
            }
            Some(Token::OpGt) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Gt,
                    right: Box::new(right),
                })
            }
            Some(Token::OpLte) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Lte,
                    right: Box::new(right),
                })
            }
            Some(Token::OpGte) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::Gte,
                    right: Box::new(right),
                })
            }
            Some(Token::KeywordIn) => {
                self.advance();
                let right = self.parse_addition()?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: BinOp::In,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn parse_addition(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_multiplication()?;
        loop {
            match self.peek_token() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: BinOp::Add,
                        right: Box::new(right),
                    };
                }
                Some(Token::Minus) => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: BinOp::Sub,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek_token() {
                Some(Token::Star) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                }
                Some(Token::Slash) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                }
                Some(Token::Percent) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp {
                        left: Box::new(left),
                        op: BinOp::Mod,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, Error> {
        match self.peek_token() {
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            Some(Token::KeywordNot) => {
                self.advance();
                if self.depth + 1 > MAX_EXPR_DEPTH {
                    return Err(Error::r#type(format!(
                        "expression nesting exceeds the limit of {MAX_EXPR_DEPTH}"
                    )));
                }
                self.depth += 1;
                let expr = self.parse_unary();
                self.depth -= 1;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr?),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, Error> {
        let (token, token_pos) = self.expect_position("expected expression")?;
        let mut expr = match token {
            Token::Ident(name) => Expr::Var(name),
            Token::Str(s) => Expr::Str(s),
            Token::Int(n) => Expr::I64(n),
            Token::Float(n) => Expr::F64(n),
            Token::Bool(b) => Expr::Bool(b),
            Token::Null => Expr::Null,
            Token::LParen => {
                let inner = self.parse_or()?;
                let (tok, pos) = self.expect_position("expected )")?;
                if !matches!(tok, Token::RParen) {
                    return Err(Error::syntax(format!("expected `)`, got {tok:?}"), pos.line, pos.column));
                }
                inner
            }
            tok => {
                return Err(Error::syntax(
                    format!("unexpected token {tok:?}"),
                    token_pos.line,
                    token_pos.column,
                ));
            }
        };

        loop {
            match self.peek_token() {
                Some(Token::Dot) => {
                    self.advance();
                    let name = self.expect_ident()?;
                    expr = Expr::Dot(Box::new(expr), name);
                }
                Some(Token::LBracket) => {
                    self.advance();
                    let index_pos = self.peek_position();
                    let index = self.parse_or()?;
                    match &index {
                        Expr::I64(n) if *n < 0 => {
                            return Err(Error::syntax(
                                "negative index is not allowed",
                                index_pos.line,
                                index_pos.column,
                            ));
                        }
                        Expr::F64(n) if *n < 0.0 => {
                            return Err(Error::syntax(
                                "negative index is not allowed",
                                index_pos.line,
                                index_pos.column,
                            ));
                        }
                        Expr::UnaryOp {
                            op: UnaryOp::Neg,
                            expr: e,
                        } if matches!(e.as_ref(), Expr::I64(_) | Expr::F64(_)) => {
                            return Err(Error::syntax(
                                "negative index is not allowed",
                                index_pos.line,
                                index_pos.column,
                            ));
                        }
                        _ => {}
                    }
                    let (tok, pos) = self.expect_position("expected ]")?;
                    if !matches!(tok, Token::RBracket) {
                        return Err(Error::syntax(format!("expected `]`, got {tok:?}"), pos.line, pos.column));
                    }
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                Some(Token::LParen) => {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek_token(), Some(Token::RParen)) {
                        args.push(self.parse_or()?);
                        while matches!(self.peek_token(), Some(Token::Comma)) {
                            self.advance();
                            args.push(self.parse_or()?);
                        }
                    }
                    let (tok, pos) = self.expect_position("expected )")?;
                    if !matches!(tok, Token::RParen) {
                        return Err(Error::syntax(format!("expected `)`, got {tok:?}"), pos.line, pos.column));
                    }
                    // Get the function name
                    match expr {
                        Expr::Var(name) => {
                            expr = Expr::Call(name, args);
                        }
                        _ => {
                            return Err(Error::syntax(
                                "function calls only supported on simple names",
                                token_pos.line,
                                token_pos.column,
                            ));
                        }
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_filter_pipes(&mut self, mut expr: Expr) -> Result<Expr, Error> {
        while matches!(self.peek_token(), Some(Token::Pipe)) {
            self.advance();
            let name_pos = self.peek_position();
            let name = self.expect_ident()?;
            let mut args = Vec::new();
            if matches!(self.peek_token(), Some(Token::LParen)) {
                self.advance();
                if !matches!(self.peek_token(), Some(Token::RParen)) {
                    while let Some(Token::Ident(_))
                    | Some(Token::Str(_))
                    | Some(Token::Int(_))
                    | Some(Token::Float(_))
                    | Some(Token::Bool(_))
                    | Some(Token::Null)
                    | Some(Token::Minus)
                    | Some(Token::KeywordNot)
                    | Some(Token::LParen) = self.peek_token()
                    {
                        args.push(self.parse_or()?);
                        if matches!(self.peek_token(), Some(Token::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                let (tok, pos) = self.expect_position("expected )")?;
                if !matches!(tok, Token::RParen) {
                    return Err(Error::syntax(format!("expected `)`, got {tok:?}"), pos.line, pos.column));
                }

                // Handle named arguments (keyword=value)
                // For now, all args are positional
            }

            // Validate the filter name and arity at parse time, so a typo fails
            // in `add_template` rather than at render time.
            match crate::filters::filter_arity(&name) {
                None => {
                    return Err(Error::undefined_filter(name, name_pos.line, name_pos.column));
                }
                Some((min, max)) => {
                    if args.len() < min || args.len() > max {
                        let expected = if min == max {
                            format!("{min}")
                        } else if max == usize::MAX {
                            format!("at least {min}")
                        } else {
                            format!("{min} to {max}")
                        };
                        return Err(Error::syntax(
                            format!("filter `{name}` expects {expected} argument(s), got {}", args.len()),
                            name_pos.line,
                            name_pos.column,
                        ));
                    }
                }
            }

            expr = Expr::Filter {
                expr: Box::new(expr),
                name,
                args,
            };
        }
        Ok(expr)
    }
}

/// Lexer over an expression that tracks absolute line/column positions in the
/// enclosing template without buffering the whole input into a `Vec<char>`.
struct Lexer<'a> {
    chars: core::iter::Peekable<core::str::Chars<'a>>,
    /// Absolute position (in the template) of the first character of the input.
    base_line: usize,
    base_col: usize,
    /// Position relative to the start of the input.
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str, base_line: usize, base_col: usize) -> Self {
        Self {
            chars: input.chars().peekable(),
            base_line,
            base_col,
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next();
        match c {
            Some('\n') => {
                self.line += 1;
                self.col = 1;
            }
            Some(_) => self.col += 1,
            None => {}
        }
        c
    }

    /// Convert the current relative position into an absolute template position.
    fn position(&self) -> SourcePosition {
        let line = self.base_line + self.line - 1;
        let column = if self.line == 1 {
            self.base_col + self.col - 1
        } else {
            self.col
        };
        SourcePosition::new(line, column)
    }
}

/// Lex an expression that started at `base_line`:`base_col` in the template, so
/// that error positions refer to the template rather than to the tag content.
pub fn lex_expr(input: &str, base_line: usize, base_col: usize) -> Result<Vec<(Token, SourcePosition)>, String> {
    let mut lx = Lexer::new(input, base_line, base_col);
    let mut tokens = Vec::new();

    while let Some(c) = lx.peek() {
        if c.is_whitespace() {
            lx.bump();
            continue;
        }

        let pos = lx.position();

        macro_rules! simple {
            ($tok:expr) => {{
                lx.bump();
                tokens.push(($tok, pos.clone()));
            }};
        }

        match c {
            '.' => simple!(Token::Dot),
            '|' => simple!(Token::Pipe),
            ',' => simple!(Token::Comma),
            ':' => simple!(Token::Colon),
            '+' => simple!(Token::Plus),
            '-' => simple!(Token::Minus),
            '*' => simple!(Token::Star),
            '/' => simple!(Token::Slash),
            '%' => simple!(Token::Percent),
            '(' => simple!(Token::LParen),
            ')' => simple!(Token::RParen),
            '[' => simple!(Token::LBracket),
            ']' => simple!(Token::RBracket),
            '=' => {
                lx.bump();
                if lx.peek() == Some('=') {
                    lx.bump();
                    tokens.push((Token::OpEq, pos));
                } else {
                    tokens.push((Token::Equals, pos));
                }
            }
            '!' => {
                lx.bump();
                if lx.peek() == Some('=') {
                    lx.bump();
                    tokens.push((Token::OpNeq, pos));
                } else {
                    return Err(format!("unexpected character `!` at {}:{}", pos.line, pos.column));
                }
            }
            '<' => {
                lx.bump();
                if lx.peek() == Some('=') {
                    lx.bump();
                    tokens.push((Token::OpLte, pos));
                } else {
                    tokens.push((Token::OpLt, pos));
                }
            }
            '>' => {
                lx.bump();
                if lx.peek() == Some('=') {
                    lx.bump();
                    tokens.push((Token::OpGte, pos));
                } else {
                    tokens.push((Token::OpGt, pos));
                }
            }
            '\'' | '"' => {
                let quote = c;
                lx.bump();
                let mut s = String::new();
                let mut closed = false;
                while let Some(ch) = lx.peek() {
                    if ch == '\\' {
                        lx.bump();
                        match lx.bump() {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some('r') => s.push('\r'),
                            Some('\\') => s.push('\\'),
                            Some('\'') => s.push('\''),
                            Some('"') => s.push('"'),
                            Some(other) => {
                                s.push('\\');
                                s.push(other);
                            }
                            None => break,
                        }
                    } else if ch == quote {
                        lx.bump();
                        closed = true;
                        break;
                    } else {
                        lx.bump();
                        s.push(ch);
                    }
                }
                if !closed {
                    return Err(format!("unclosed string literal at {}:{}", pos.line, pos.column));
                }
                tokens.push((Token::Str(s), pos));
            }
            _ if c.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(d) = lx.peek() {
                    if d.is_ascii_digit() {
                        num.push(d);
                        lx.bump();
                    } else {
                        break;
                    }
                }
                let mut is_float = false;
                if lx.peek() == Some('.') {
                    is_float = true;
                    num.push('.');
                    lx.bump();
                    while let Some(d) = lx.peek() {
                        if d.is_ascii_digit() {
                            num.push(d);
                            lx.bump();
                        } else {
                            break;
                        }
                    }
                }
                if is_float {
                    let n: f64 = num.parse().map_err(|e| format!("bad float: {e}"))?;
                    tokens.push((Token::Float(n), pos));
                } else {
                    let n: i64 = num.parse().map_err(|e| format!("bad int: {e}"))?;
                    tokens.push((Token::Int(n), pos));
                }
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(ch) = lx.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        lx.bump();
                    } else {
                        break;
                    }
                }
                let token = match ident.as_str() {
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    "null" | "none" | "nil" => Token::Null,
                    "and" => Token::KeywordAnd,
                    "or" => Token::KeywordOr,
                    "not" => Token::KeywordNot,
                    "in" => Token::KeywordIn,
                    _ => Token::Ident(ident),
                };
                tokens.push((token, pos));
            }
            _ => {
                return Err(format!("unexpected character `{c}` at {}:{}", pos.line, pos.column));
            }
        }
    }

    Ok(tokens)
}

/// The statically-known type of an expression, when it can be determined without
/// evaluation. Used to reject obvious type errors at parse time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StaticType {
    Bool,
    Str,
    Int,
    Float,
    Null,
    Array,
    Unknown,
}

/// Statically infer the type of `expr`, returning [`StaticType::Unknown`] when it
/// depends on runtime values.
pub fn static_type(expr: &Expr) -> StaticType {
    match expr {
        Expr::Var(_) | Expr::Dot(..) | Expr::Index(..) => StaticType::Unknown,
        Expr::Str(_) => StaticType::Str,
        Expr::I64(_) => StaticType::Int,
        Expr::F64(_) => StaticType::Float,
        Expr::Bool(_) => StaticType::Bool,
        Expr::Null => StaticType::Null,
        Expr::Filter {
            ..
        } => StaticType::Unknown,
        Expr::Call(name, _) => match name.as_str() {
            "is_set" => StaticType::Bool,
            "range" => StaticType::Array,
            _ => StaticType::Unknown,
        },
        Expr::BinOp {
            left,
            op,
            right,
        } => match op {
            BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte | BinOp::In => StaticType::Bool,
            BinOp::And | BinOp::Or => {
                if static_type(left) == StaticType::Bool && static_type(right) == StaticType::Bool {
                    StaticType::Bool
                } else {
                    StaticType::Unknown
                }
            }
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => StaticType::Unknown,
        },
        Expr::UnaryOp {
            op,
            expr,
        } => match op {
            UnaryOp::Not => StaticType::Bool,
            UnaryOp::Neg => static_type(expr),
        },
    }
}

fn type_name(t: StaticType) -> &'static str {
    match t {
        StaticType::Bool => "a boolean",
        StaticType::Str => "a string",
        StaticType::Int => "an integer",
        StaticType::Float => "a float",
        StaticType::Null => "null",
        StaticType::Array => "an array",
        StaticType::Unknown => "an unknown value",
    }
}

/// Check that `expr` is statically known to be a boolean, or dynamic (in which
/// case the check happens at render time).
pub fn check_static_bool(expr: &Expr, context: &str) -> Result<(), Error> {
    match static_type(expr) {
        StaticType::Bool | StaticType::Unknown => Ok(()),
        other => Err(Error::r#type(format!(
            "expected a boolean in {context}, but found {}",
            type_name(other)
        ))),
    }
}

/// Validate boolean operands (`and`, `or`, `not`) throughout an expression at
/// parse time.
pub fn validate_types(expr: &Expr) -> Result<(), Error> {
    match expr {
        Expr::BinOp {
            left,
            op: BinOp::And,
            right,
        } => {
            check_static_bool(left, "`and`")?;
            validate_types(left)?;
            validate_types(right)
        }
        Expr::BinOp {
            left,
            op: BinOp::Or,
            right,
        } => {
            check_static_bool(left, "`or`")?;
            validate_types(left)?;
            validate_types(right)
        }
        Expr::UnaryOp {
            op: UnaryOp::Not,
            expr,
        } => {
            check_static_bool(expr, "`not`")?;
            validate_types(expr)
        }
        Expr::BinOp {
            left,
            right,
            ..
        } => {
            validate_types(left)?;
            validate_types(right)
        }
        Expr::UnaryOp {
            expr, ..
        } => validate_types(expr),
        Expr::Dot(inner, _) => validate_types(inner),
        Expr::Index(inner, index) => {
            validate_types(inner)?;
            validate_types(index)
        }
        Expr::Filter {
            expr,
            args,
            ..
        } => {
            validate_types(expr)?;
            for a in args {
                validate_types(a)?;
            }
            Ok(())
        }
        Expr::Call(_, args) => {
            for a in args {
                validate_types(a)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
