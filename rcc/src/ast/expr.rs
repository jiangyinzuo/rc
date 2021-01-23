use std::fmt;
use std::fmt::{Debug, Formatter, Write};

use crate::lexer::token::Token::{Minus, Not, Star};
use crate::lexer::token::{Token};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Path(PathExpr),
    Lit(LitExpr),
    Unary(UnAryExpr),
    Block(BlockExpr),
    Nothing,
}

#[derive(Debug, PartialEq)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

#[derive(PartialEq, Debug)]
pub struct LitExpr {
    pub ret_type: String,
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct PathExpr {
    pub segments: Vec<String>,
}

impl PathExpr {
    pub fn new() -> Self {
        PathExpr { segments: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnAryExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
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
