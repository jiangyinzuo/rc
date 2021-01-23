use crate::lexer::token::Token::{LeftCurlyBraces, RightCurlyBraces};

use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr;
use crate::parser::Parse;
use crate::parser::ParseCursor;
use crate::rcc::RccError;

impl<'a> Parse<'a> for BlockExpr {
    fn parse(cxt: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        if cxt.bump_token()? == &LeftCurlyBraces {
            let mut block_expr = BlockExpr { exprs: vec![] };
            while cxt.next_token()? != &RightCurlyBraces {
                block_expr.exprs.push(Expr::parse(cxt)?);
            }
            cxt.bump_token();
            Ok(block_expr)
        } else {
            Err("invalid block_expr".into())
        }
    }
}