use crate::parser::expr::Expr;
use crate::{Parse, ParseContext};
use lexer::token::Token::{LeftCurlyBraces, RightCurlyBraces};

#[derive(Debug, PartialEq)]
pub struct BlockExpr<'a> {
    pub exprs: Vec<Expr<'a>>
}

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