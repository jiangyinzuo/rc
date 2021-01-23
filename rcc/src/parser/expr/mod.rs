use crate::lexer::token::Token::*;

use crate::ast::expr::Expr;
use crate::ast::expr::Expr::*;
use crate::ast::expr::LitExpr;
use crate::ast::expr::PathExpr;
use crate::ast::expr::UnAryExpr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

pub mod block_expr;
pub mod lit_expr;
pub mod path_expr;
pub mod unary_expr;

impl<'a> Parse<'a> for Expr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        match cursor.next_token()? {
            Not | Star | Minus => Ok(Unary(UnAryExpr::parse(cursor)?)),
            Identifier(_) | PathSep => {
                let path = Path(PathExpr::parse(cursor)?);
                Ok(path)
            }
            Literal { .. } => Ok(Lit(LitExpr::parse(cursor)?)),
            Semi => Ok(Nothing),
            _ => Err("invalid expr".into()),
        }
    }
}
