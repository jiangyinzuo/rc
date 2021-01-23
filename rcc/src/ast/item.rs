use crate::ast::expr::BlockExpr;
use crate::ast::types::{Type};
use crate::ast::Visibility;

#[derive(Debug, PartialEq)]
pub struct ItemFn {
    pub name: String,
    pub ret_type: Type,
    pub fn_block: Option<BlockExpr>
}

impl ItemFn {
    pub  fn new(name: String, ret_type: Type, fn_block: BlockExpr) ->Self {
        ItemFn {
            name,
            ret_type,
            fn_block: Some(fn_block)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VisItem {
    pub vis: Visibility,
    pub inner_item: InnerItem,
}

impl VisItem {
    pub fn new(vis: Visibility, inner_item: InnerItem) -> Self {
        VisItem {
            vis,
            inner_item
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InnerItem {
    /// fn add(a, b) { a + b }
    Fn(ItemFn),

    /// struct Foo { x: i32 }
    Struct(ItemStruct),

    /// enum Color { Red, Yellow }
    Enum(TypeEnum),

    /// const A: i32 = 2;
    Const,

    /// static B: i32 = 3;
    Static,

    /// impl Foo { ... }
    Impl,
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
pub struct TypeEnum {
    name: String,
    enum_items: Vec<EnumItem>,
}

#[derive(Debug, PartialEq)]
pub struct EnumItem {
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
    pub _type: Type
}
