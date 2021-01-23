use crate::ast::types::Type::Tuple;
use std::fmt::{Debug, Formatter};

#[derive(PartialEq)]
pub enum Type {
    /// `char`, `u8`, `bool`,
    ///  `struct Foo;`, `enum Color(String);`, etc.
    Identifier(String),

    /// `()`, `(i32, char)`, ...
    /// `()` is also called unit type
    Tuple(TypeTuple),

    /// `[i32; 3]`
    Array(TypeArray),

    /// `[i32]`
    Slice(TypeSlice),

    /// `fn (i32, i32) -> i32`
    FnPtr(TypeFnPtr),

    Ptr(TypePtr),
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(s) => f.write_str(&s),
            Self::Tuple(tp) => write!(f, "({:?})", tp),
            Self::Array(ta) => write!(f, "[{:?}; {}]", ta._type, ta.len),
            Self::Slice(ts) => write!(f, "[{:?}]", ts),
            Self::FnPtr(fptr) => write!(f, "{:?}", fptr),
            Self::Ptr(ptr) => write!(f, "{:?}", ptr),
        }
    }
}

impl Type {
    pub fn unit() -> Type {
        Tuple(vec![])
    }
}

pub type TypeTuple = Vec<Type>;
pub type TypeSlice = Box<Type>;

#[derive(Debug, PartialEq)]
pub struct TypeArray {
    _type: Box<Type>,
    len: u32,
}

#[derive(Debug, PartialEq)]
pub struct TypeFnPtr {
    args: Vec<Type>,
    ret_type: Box<Type>,
}

#[derive(Debug, PartialEq)]
pub enum PtrKind {
    /// &i32
    Ref,
    /// &mut i32
    MutRef,
    /// *i32
    RawPtr,
    /// *mut i32
    MutRawPtr,
    /// *const i32
    ConstRawPtr,
}

#[derive(Debug, PartialEq)]
pub struct TypePtr {
    ptr_kind: PtrKind,
    _type: Box<Type>,
}
