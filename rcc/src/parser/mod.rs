//! Notation        | Examples        | Meaning
//! -----------------------------------------------------------------
//! snake_case `x`  | fn, `|`         | A token produced by the lexer
//! ItalicCamelCase | VisItem, Item   | A syntactical production
//! x?              | pub?            | An optional item
//! x*              | OuterAttribute* | 0 or more of x
//! x+              | MacroMatch+     | 1 or more of x
//! |               | u8 | u16, Block | Item Either one or another
//! ( )             | (, Parameter)?  | Groups items
//! -----------------------------------------------------------------
//!
//! `Syntactical Productions:`
//!
//! File -> Item File | Item
//! Item -> pub? VisItem | Impl
//! VisItem -> Fn | Struct | Enum | Const | Static
//!
//! Fn -> FnSignature BlockExpr
//! FnSignature -> fn Ident `(` FnArgs? `)` RetType?
//! FnArgs -> FnArg (`,` FnArg)* `,`?
//! RetType -> r_arrow Type
//! Type -> ident | `()` | ( left_paren (Type comma)+ Type? right_paren ) |
//!         bool | char |
//!         f32 | f64 | i8 | i16 | i32 | i64 |
//!         i128 | isize | u8 | u16 | u32 | u64 | u128 | usize
//!
//! Static -> static ident TypeAnnotation eq semi

mod expr;
mod file;
mod item;

pub enum Visibility {
    Pub,
    Priv,
}

pub struct TypeAnnotation {}

pub enum Type<'a> {
    UserDefined(&'a str),
    Unit,
    Tuple(Vec<Type<'a>>),
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}
