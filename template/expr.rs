//! Expression lexer and recursive-descent parser for template expressions.

use alloc::{boxed::Box, format, string::String, vec::Vec};

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
}

impl ExprParser {
    pub fn new(tokens: Vec<(Token, SourcePosition)>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    pub fn parse_all(mut self) -> Result<Expr, Error> {
        let expr = self.parse_or()?;
        if self.pos < self.tokens.len() {
            let (_, pos) = &self.tokens[self.pos];
            return Err(Error::syntax("unexpected trailing tokens", pos.line, pos.column));
        }
        Ok(expr)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _)| t)
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
        self.advance().cloned().ok_or_else(|| Error::syntax(msg, 0, 0))
    }

    fn expect_ident(&mut self) -> Result<String, Error> {
        match self.expect()? {
            Token::Ident(s) => Ok(s),
            tok => Err(Error::parse(format!("expected identifier, got {tok:?}"))),
        }
    }

    fn parse_or(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_and()?;
        while matches!(self.peek_token(), Some(Token::KeywordOr)) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }
        self.parse_filter_pipes(left)
    }

    fn parse_and(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek_token(), Some(Token::KeywordAnd)) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
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
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, Error> {
        let token = self.expect_position("expected expression")?;
        let mut expr = match token.0 {
            Token::Ident(name) => Expr::Var(name),
            Token::Str(s) => Expr::Str(s),
            Token::Int(n) => Expr::I64(n),
            Token::Float(n) => Expr::F64(n),
            Token::Bool(b) => Expr::Bool(b),
            Token::Null => Expr::Null,
            Token::LParen => {
                let inner = self.parse_or()?;
                match self.expect_position("expected )")?.0 {
                    Token::RParen => {}
                    tok => return Err(Error::syntax(format!("expected `)`, got {tok:?}"), 0, 0)),
                }
                inner
            }
            tok => return Err(Error::syntax(format!("unexpected token {tok:?}"), 0, 0)),
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
                    let index = self.parse_or()?;
                    match &index {
                        Expr::I64(n) if *n < 0 => {
                            let (_, pos) = &self.tokens[0];
                            return Err(Error::syntax("negative index is not allowed", pos.line, pos.column));
                        }
                        Expr::F64(n) if *n < 0.0 => {
                            let (_, pos) = &self.tokens[0];
                            return Err(Error::syntax("negative index is not allowed", pos.line, pos.column));
                        }
                        Expr::UnaryOp {
                            op: UnaryOp::Neg,
                            expr: e,
                        } if matches!(e.as_ref(), Expr::I64(_) | Expr::F64(_)) => {
                            let (_, pos) = &self.tokens[0];
                            return Err(Error::syntax("negative index is not allowed", pos.line, pos.column));
                        }
                        _ => {}
                    }
                    match self.expect_position("expected ]")?.0 {
                        Token::RBracket => {}
                        tok => return Err(Error::syntax(format!("expected `]`, got {tok:?}"), 0, 0)),
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
                    match self.expect_position("expected )")?.0 {
                        Token::RParen => {}
                        tok => return Err(Error::syntax(format!("expected `)`, got {tok:?}"), 0, 0)),
                    }
                    // Get the function name
                    match expr {
                        Expr::Var(name) => {
                            expr = Expr::Call(name, args);
                        }
                        _ => return Err(Error::syntax("function calls only supported on simple names", 0, 0)),
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
            let name = self.expect_ident()?;
            let mut args = Vec::new();
            if matches!(self.peek_token(), Some(Token::LParen)) {
                self.advance();
                if !matches!(self.peek_token(), Some(Token::RParen)) {
                    loop {
                        match self.peek_token() {
                            Some(Token::Ident(_))
                            | Some(Token::Str(_))
                            | Some(Token::Int(_))
                            | Some(Token::Float(_))
                            | Some(Token::Bool(_))
                            | Some(Token::Null)
                            | Some(Token::Minus)
                            | Some(Token::KeywordNot)
                            | Some(Token::LParen) => {
                                args.push(self.parse_or()?);
                            }
                            _ => break,
                        }
                        if matches!(self.peek_token(), Some(Token::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                match self.expect_position("expected )")?.0 {
                    Token::RParen => {}
                    tok => return Err(Error::syntax(format!("expected `)`, got {tok:?}"), 0, 0)),
                }

                // Handle named arguments (keyword=value)
                // For now, all args are positional
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

pub fn lex_expr(input: &str) -> Result<Vec<(Token, SourcePosition)>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut line = 1;
    let mut col = 1;

    while i < chars.len() {
        let c = chars[i];
        let start_col = col;

        if c.is_whitespace() {
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            i += 1;
            continue;
        }

        match c {
            '.' => {
                tokens.push((Token::Dot, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '|' => {
                tokens.push((Token::Pipe, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            ',' => {
                tokens.push((Token::Comma, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            ':' => {
                tokens.push((Token::Colon, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push((Token::OpEq, SourcePosition::new(line, col)));
                    i += 2;
                    col += 2;
                } else {
                    tokens.push((Token::Equals, SourcePosition::new(line, col)));
                    i += 1;
                    col += 1;
                }
            }
            '(' => {
                tokens.push((Token::LParen, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            ')' => {
                tokens.push((Token::RParen, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '[' => {
                tokens.push((Token::LBracket, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            ']' => {
                tokens.push((Token::RBracket, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '+' => {
                tokens.push((Token::Plus, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '-' => {
                tokens.push((Token::Minus, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '*' => {
                tokens.push((Token::Star, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '/' => {
                tokens.push((Token::Slash, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '%' => {
                tokens.push((Token::Percent, SourcePosition::new(line, col)));
                i += 1;
                col += 1;
            }
            '!' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push((Token::OpNeq, SourcePosition::new(line, col)));
                    i += 2;
                    col += 2;
                } else {
                    return Err(format!("unexpected character `!` at {line}:{col}"));
                }
            }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push((Token::OpLte, SourcePosition::new(line, col)));
                    i += 2;
                    col += 2;
                } else {
                    tokens.push((Token::OpLt, SourcePosition::new(line, col)));
                    i += 1;
                    col += 1;
                }
            }
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push((Token::OpGte, SourcePosition::new(line, col)));
                    i += 2;
                    col += 2;
                } else {
                    tokens.push((Token::OpGt, SourcePosition::new(line, col)));
                    i += 1;
                    col += 1;
                }
            }
            '\'' | '"' => {
                let quote = c;
                let mut s = String::new();
                i += 1;
                col += 1;
                while i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        let next = chars[i + 1];
                        match next {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '\\' => s.push('\\'),
                            '\'' => s.push('\''),
                            '"' => s.push('"'),
                            c => {
                                s.push('\\');
                                s.push(c);
                            }
                        }
                        i += 2;
                        col += 2;
                    } else if chars[i] == quote {
                        i += 1;
                        col += 1;
                        break;
                    } else {
                        if chars[i] == '\n' {
                            line += 1;
                            col = 1;
                        } else {
                            col += 1;
                        }
                        s.push(chars[i]);
                        i += 1;
                    }
                }
                tokens.push((Token::Str(s), SourcePosition::new(line, start_col)));
            }
            _ if c.is_ascii_digit() || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) => {
                let mut num = String::new();
                if c == '-' {
                    num.push('-');
                    i += 1;
                    col += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num.push(chars[i]);
                    i += 1;
                    col += 1;
                }
                let mut is_float = false;
                if i < chars.len() && chars[i] == '.' {
                    is_float = true;
                    num.push('.');
                    i += 1;
                    col += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        num.push(chars[i]);
                        i += 1;
                        col += 1;
                    }
                }
                if is_float {
                    let n: f64 = num.parse().map_err(|e| format!("bad float: {e}"))?;
                    tokens.push((Token::Float(n), SourcePosition::new(line, start_col)));
                } else {
                    let n: i64 = num.parse().map_err(|e| format!("bad int: {e}"))?;
                    tokens.push((Token::Int(n), SourcePosition::new(line, start_col)));
                }
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    ident.push(chars[i]);
                    i += 1;
                    col += 1;
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
                tokens.push((token, SourcePosition::new(line, start_col)));
            }
            _ => {
                return Err(format!("unexpected character `{c}` at {line}:{col}"));
            }
        }
    }

    Ok(tokens)
}
