use lexer::token::Token::*;

use crate::{Parse, ParseContext};
use crate::parser::expr::Expr::*;
use crate::parser::expr::lit_expr::LitExpr;
use crate::parser::expr::path_expr::PathExpr;
use crate::parser::expr::unary_expr::UnAryExpr;
use crate::parser::expr::block_expr::BlockExpr;

pub mod lit_expr;
pub mod path_expr;
pub mod unary_expr;
pub mod block_expr;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Path(PathExpr<'a>),
    Lit(LitExpr<'a>),
    Unary(UnAryExpr<'a>),
    Block(BlockExpr<'a>),
    Nothing,
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        match cxt.next_token()? {
            Not | Star | Minus => Ok(Unary(UnAryExpr::parse(cxt)?)),
            Identifier(_) | PathSep => {
                let path = Path(PathExpr::parse(cxt)?);
                Ok(path)
            }
            Literal{ .. } => Ok(Lit(LitExpr::parse(cxt)?)),
            Semi => Ok(Nothing),
            _ => Err("invalid expr"),
        }
    }
}
