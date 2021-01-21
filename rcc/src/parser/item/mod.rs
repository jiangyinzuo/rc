use lexer::token::Token;

use crate::ast::item::{Item, ItemFn};
use crate::parser::Parse;
use crate::parser::ParseContext;

pub mod item_fn;

impl <'a> Parse<'a> for Item<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
         match cxt.next_token()? {
             Token::Fn => Ok(Self::Fn(ItemFn::parse(cxt)?)),
             _ => Err("invalid item")
         }
    }
}