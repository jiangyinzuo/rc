use crate::lexer::token::Token::{Minus, Not, Star};

use crate::ast::expr::Expr;
use crate::ast::expr::{UnAryExpr, UnOp};
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl<'a> Parse<'a> for UnAryExpr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        let tk = cursor.eat_token_in(&[Not, Star, Minus])?;
        let op = UnOp::from_token(tk).unwrap();
        let expr = Expr::parse(cursor)?;
        Ok(UnAryExpr {
            op,
            expr: Box::new(expr),
        })
    }
}
