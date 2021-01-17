mod item_fn;

use crate::parser::item::item_fn::ItemFn;
use lexer::token::Token;

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
