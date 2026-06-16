//! AST types for the template engine — nodes, expressions, and operators.

use alloc::{boxed::Box, string::String, vec::Vec};

pub type NodeList = Vec<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    Raw(String),
    Expr(Expr),
    If(IfNode),
    For(ForNode),
    Include(String),
    Extends(String),
    Block(BlockNode),
    Set(String, Expr),
    RawBlock(String),
}

#[derive(Clone, Debug)]
pub struct IfNode {
    pub condition: Expr,
    pub body: NodeList,
    pub elifs: Vec<ElifNode>,
    pub else_body: Option<NodeList>,
}

#[derive(Clone, Debug)]
pub struct ElifNode {
    pub condition: Expr,
    pub body: NodeList,
}

#[derive(Clone, Debug)]
pub struct ForNode {
    pub var_name: String,
    pub iterable: Expr,
    pub body: NodeList,
}

#[derive(Clone, Debug)]
pub struct BlockNode {
    pub name: String,
    pub body: NodeList,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(String),
    Dot(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    Str(String),
    I64(i64),
    F64(f64),
    Bool(bool),
    Null,
    Filter {
        expr: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call(String, Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    In,
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Not,
    Neg,
}
