use std::fmt;
use std::fmt::{Debug, Formatter, Write};

use crate::lexer::token::Token;
use crate::lexer::token::Token::{Minus, Not, Star};

use crate::ast::expr::{UnAryExpr, UnOp};
use crate::ast::expr::Expr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl<'a> Parse<'a> for UnAryExpr {
    fn parse(cxt: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let tk = cxt.bump_token()?;
        if matches!(tk, Not | Star | Minus) {
            let op = UnOp::from_token(tk).unwrap();
            let expr = Expr::parse(cxt)?;
            Ok(UnAryExpr {
                op,
                expr: Box::new(expr),
            })
        } else {
            Err("invalid unary expr".into())
        }
    }
}
