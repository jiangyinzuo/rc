use lexer::token::Token;
use lexer::token::Token::RArrow;

use crate::ast::item::ItemFn;
use crate::parser::{Parse, Visibility};
use crate::ast::expr::BlockExpr;
use crate::ast::expr::Expr::Block;
use crate::parser::ParseContext;
use crate::ast::type_anno::TypeAnno;
use crate::ast::type_anno::TypeAnno::Origin;

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