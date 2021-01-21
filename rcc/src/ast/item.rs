use crate::ast::expr::BlockExpr;

#[derive(Debug, PartialEq)]
pub struct ItemFn<'a> {
    pub ident: &'a str,
    pub ret_type: &'a str,
    pub fn_block: Option<BlockExpr<'a>>
}

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
