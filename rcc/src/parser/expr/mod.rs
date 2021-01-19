use crate::parser::expr::lit_expr::LitExpr;
use crate::parser::expr::path_expr::PathExpr;
use crate::parser::expr::unary_expr::UnAryExpr;
use crate::parser::expr::Expr::*;
use crate::{Parse, ParseContext};
use lexer::token::Token::*;

mod lit_expr;
mod path_expr;
mod tests;
mod unary_expr;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Path(PathExpr<'a>),
    Lit(LitExpr<'a>),
    Unary(UnAryExpr<'a>),
    Nothing,
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if let Some(tk) = cxt.next_token() {
            return match tk {
                Not | Star | Minus => Ok(Unary(UnAryExpr::parse(cxt)?)),
                Identifier(_) | PathSep => {
                    let path = Path(PathExpr::parse(cxt)?);
                    Ok(path)
                }
                Semi => Ok(Nothing),
                _ => Err("invalid expr"),
            };
        }
        Err("invalid expr")
    }
}
