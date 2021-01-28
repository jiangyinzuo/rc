use crate::ast::expr::BlockExpr;
use crate::ast::pattern::Pattern;
use crate::ast::types::Type;
use crate::ast::{NamedASTNode, TokenStart, Visibility};
use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub struct ItemFn {
    pub name: String,
    pub fn_params: FnParams,
    pub ret_type: Type,
    pub fn_block: Option<BlockExpr>,
}

impl ItemFn {
    pub fn new(name: String, fn_params: FnParams, ret_type: Type, fn_block: BlockExpr) -> Self {
        ItemFn {
            name,
            fn_params,
            ret_type,
            fn_block: Some(fn_block),
        }
    }
}

pub type FnParams = Vec<FnParam>;

#[derive(Debug, PartialEq)]
pub struct FnParam {
    pattern: Pattern,
    _type: Type,
}

impl FnParam {
    pub fn new(pattern: Pattern, _type: Type) -> Self {
        FnParam { pattern, _type }
    }
}

impl TokenStart for FnParam {
    fn is_token_start(tk: &Token) -> bool {
        Pattern::is_token_start(tk)
    }
}

impl NamedASTNode for ItemFn {
    fn ident_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq)]
pub struct VisItem {
    pub vis: Visibility,
    pub inner_item: InnerItem,
}

impl VisItem {
    pub fn new(vis: Visibility, inner_item: InnerItem) -> Self {
        VisItem { vis, inner_item }
    }
}

impl TokenStart for VisItem {
    fn is_token_start(tk: &Token) -> bool {
        tk == &Token::Pub || InnerItem::is_token_start(tk)
    }
}

impl NamedASTNode for VisItem {
    fn ident_name(&self) -> &str {
        self.inner_item.ident_name()
    }
}

#[derive(Debug, PartialEq)]
pub enum InnerItem {
    /// fn add(a, b) { a + b }
    Fn(ItemFn),

    /// struct Foo { x: i32 }
    Struct(ItemStruct),

    /// enum Color { Red, Yellow }
    Enum(ItemEnum),

    /// const A: i32 = 2;
    Const,

    /// static B: i32 = 3;
    Static,

    /// impl Foo { ... }
    Impl,

    /// extern "C" {}
    ExternalBlock,
}

impl TokenStart for InnerItem {
    fn is_token_start(tk: &Token) -> bool {
        matches!(
            tk,
            Token::Priv
                | Token::Fn
                | Token::Const
                | Token::Static
                | Token::Struct
                | Token::Enum
                | Token::Impl
        )
    }
}

impl NamedASTNode for InnerItem {
    fn ident_name(&self) -> &str {
        match self {
            Self::Fn(item_fn) => item_fn.ident_name(),
            _ => unimplemented!(),
        }
    }
}

/// # Examples
/// `struct Student { name: String, age: u32 }`
/// `struct Teacher(String, u32);`
#[derive(Debug, PartialEq)]
pub struct ItemStruct {
    name: String,
    fields: Fields,
}

impl ItemStruct {
    pub fn new(name: String) -> Self {
        ItemStruct {
            name,
            fields: Fields::None,
        }
    }

    pub fn struct_fields(mut self, struct_fields: Vec<StructField>) -> Self {
        self.fields = Fields::Struct(struct_fields);
        self
    }

    pub fn tuple_fields(mut self, tuple_fields: Vec<TupleField>) -> Self {
        self.fields = Fields::Tuple(tuple_fields);
        self
    }
}

/// enum Identity {
///     Student { name: String },
///     Teacher(String),
///     Admin,
/// }
#[derive(Debug, PartialEq)]
pub struct ItemEnum {
    name: String,
    enum_items: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    name: String,
    fields: Fields,
}

#[derive(Debug, PartialEq)]
pub enum Fields {
    /// `struct Foo {a: i32}`
    Struct(Vec<StructField>),
    /// `struct Foo(i32);`
    Tuple(Vec<TupleField>),
    /// `struct Foo;`
    None,
}

#[derive(Debug, PartialEq)]
pub struct StructField {
    pub vis: Visibility,
    pub name: String,
    pub _type: Type,
}

#[derive(Debug, PartialEq)]
pub struct TupleField {
    pub vis: Visibility,
    pub _type: Type,
}