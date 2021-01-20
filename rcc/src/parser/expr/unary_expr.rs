use crate::parser::expr::Expr;
use crate::{Parse, ParseContext};
use lexer::token::Token;
use lexer::token::Token::{Minus, Not, Star};
use std::fmt;
use std::fmt::{Debug, Formatter, Write};

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
    fn from_token(tk: &Token) -> Option<Self> {
        match tk {
            Minus => Some(Self::Neg),
            Star => Some(Self::Deref),
            Not => Some(Self::Not),
            _ => None,
        }
    }
}

impl<'a> Parse<'a> for UnAryExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        let tk = cxt.bump_token()?;
        if matches!(tk, Not | Star | Minus) {
            let op = UnOp::from_token(tk).unwrap();
            let expr = Expr::parse(cxt)?;
            Ok(UnAryExpr {
                op,
                expr: Box::new(expr),
            })
        } else {
            Err("invalid unary expr")
        }
    }
}
