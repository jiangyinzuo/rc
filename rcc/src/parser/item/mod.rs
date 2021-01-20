use lexer::token::Token;

use crate::parser::ParseContext;
use crate::ast::item::ItemFn;
use crate::parser::Parse;

pub mod item_fn;

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    /// fn add(a, b) { a + b }
    Fn(ItemFn<'a>),

    /// struct Foo { x: i32 }
    Struct,

    /// enum Color { Red, Yellow }
    Enum,

    /// const A: i32 = 2;
    Const,

    /// static B: i32 = 3;
    Static,

    /// impl Foo { ... }
    Impl,
}

impl <'a> Parse<'a> for Item<'a> {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str> {
         match cxt.next_token()? {
             Token::Fn => Ok(Self::Fn(ItemFn::parse(cxt)?)),
             _ => Err("invalid item")
         }
    }
}