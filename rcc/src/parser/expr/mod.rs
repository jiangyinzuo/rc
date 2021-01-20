use lexer::token::Token::*;

use crate::parser::ParseContext;
use crate::ast::expr::BlockExpr;
use crate::parser::expr::Expr::*;
use crate::ast::expr::LitExpr;
use crate::ast::expr::PathExpr;
use crate::ast::expr::UnAryExpr;
use crate::parser::Parse;

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
