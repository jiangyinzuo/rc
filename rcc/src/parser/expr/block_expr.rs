use lexer::token::Token::{LeftCurlyBraces, RightCurlyBraces};

use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr;
use crate::parser::Parse;
use crate::parser::ParseContext;

impl<'a> Parse<'a> for BlockExpr<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if cxt.bump_token()? == &LeftCurlyBraces {
            let mut block_expr = BlockExpr { exprs: vec![] };
            while cxt.next_token()? != &RightCurlyBraces {
                block_expr.exprs.push(Expr::parse(cxt)?);
            }
            cxt.bump_token();
            Ok(block_expr)
        } else {
            Err("invalid block_expr")
        }
    }
}