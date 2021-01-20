use crate::parser::Visibility;
use crate::{Parse, ParseContext};
use lexer::token::Token;
use lexer::token::Token::RArrow;
use crate::parser::type_anno::TypeAnno;
use crate::parser::type_anno::TypeAnno::Origin;
use crate::parser::expr::block_expr::BlockExpr;
use crate::parser::expr::Expr::Block;

#[derive(Debug, PartialEq)]
pub struct ItemFn<'a> {
    pub ident: &'a str,
    pub ret_type: &'a str,
    pub fn_block: Option<BlockExpr<'a>>
}

impl<'a> Parse<'a> for ItemFn<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
        if cxt.bump_token()? == &Token::Fn {
            if let Token::Identifier(ident) = cxt.bump_token()? {
                let ident = *ident;
                if cxt.bump_token()? == &Token::LeftParen && cxt.bump_token()? == &Token::RightParen {
                    if cxt.next_token()? == &RArrow {
                        cxt.bump_token();
                        if let Origin(ret_type) = TypeAnno::parse(cxt)? {
                            let fn_block = BlockExpr::parse(cxt)?;
                            return Ok(ItemFn { ident, ret_type, fn_block: Some(fn_block) });
                        }
                    }
                }
            }
        }
        Err("invalid item_fn")
    }
}