use std::fmt;
use std::fmt::{Debug, Formatter, Write};

use crate::lexer::token::Token::{Minus, Not, Star};
use crate::lexer::token::{LiteralKind, Token};

trait Type {
    fn ret_type(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Path(PathExpr<'a>),
    Lit(LitExpr<'a>),
    Unary(UnAryExpr<'a>),
    Block(BlockExpr<'a>),
    Nothing,
}

impl<'a> Type for Expr<'a> {
    fn ret_type(&self) -> String {
        match self {
            Self::Lit(lit_expr) => lit_expr.ret_type(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockExpr<'a> {
    pub exprs: Vec<Expr<'a>>,
}

impl<'a> Type for BlockExpr<'a> {
    fn ret_type(&self) -> String {
        if let Some(expr) = self.exprs.last() {
            expr.ret_type()
        } else {
            "()".to_string()
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LitExpr<'a> {
    pub ret_type: &'a str,
    pub value: &'a str,
}

impl<'a> Type for LitExpr<'a> {
    fn ret_type(&self) -> String {
        self.ret_type.to_string()
    }
}

#[derive(PartialEq, Debug)]
pub struct PathExpr<'a> {
    pub segments: Vec<&'a str>,
}

impl<'a> PathExpr<'a> {
    pub fn new() -> Self {
        PathExpr { segments: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnAryExpr<'a> {
    pub op: UnOp,
    pub expr: Box<Expr<'a>>,
}

#[derive(PartialEq)]
pub enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

impl Debug for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::Deref => '*',
            Self::Not => '!',
            Self::Neg => '-',
        })
    }
}

impl UnOp {
    pub(crate) fn from_token(tk: &Token) -> Option<Self> {
        match tk {
            Minus => Some(Self::Neg),
            Star => Some(Self::Deref),
            Not => Some(Self::Not),
            _ => None,
        }
    }
}
