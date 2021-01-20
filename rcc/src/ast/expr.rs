use std::fmt::{Debug, Formatter, Write};
use std::fmt;

use lexer::token::{LiteralKind, Token};
use lexer::token::Token::{Minus, Not, Star};

use crate::parser::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct BlockExpr<'a> {
    pub exprs: Vec<Expr<'a>>
}

#[derive(PartialEq, Debug)]
pub struct LitExpr<'a> {
    pub literal_kind: LiteralKind<'a>,
    pub value: &'a str,
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
