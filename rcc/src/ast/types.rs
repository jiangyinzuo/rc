use std::fmt::{Debug, Formatter};
use strenum::StrEnum;
use crate::ast::item::ItemFn;
use crate::ast::types::TypeAnnotation::{Identifier, Tuple};

#[derive(PartialEq, Clone)]
pub enum TypeAnnotation {
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

    /// !
    Never,

    Bool,
    Str,
    Char,
    LitNum(TypeLitNum),

    Unknown,
}

impl From<String> for TypeAnnotation {
    fn from(s: String) -> Self {
        Identifier(s)
    }
}

impl From<&str> for TypeAnnotation {
    fn from(s: &str) -> Self {
        Identifier(s.into())
    }
}

impl Debug for TypeAnnotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(s) => f.write_str(&s),
            Self::Tuple(tp) => write!(f, "({:?})", tp),
            Self::Array(ta) => write!(f, "[{:?}; {}]", ta._type, ta.len),
            Self::Slice(ts) => write!(f, "[{:?}]", ts),
            Self::FnPtr(fptr) => write!(f, "{:?}", fptr),
            Self::Ptr(ptr) => write!(f, "{:?}", ptr),
            Self::Never => write!(f, "!"),
            Self::Bool => write!(f, "bool"),
            Self::Str => write!(f, "&str"),
            Self::Char => write!(f, "char"),
            Self::LitNum(tl) => write!(f, "{:?}", tl),
            Self::Unknown => write!(f, "[unknown]")
        }
    }
}

impl TypeAnnotation {
    pub fn unit() -> TypeAnnotation {
        Tuple(vec![])
    }
}

pub type TypeTuple = Vec<TypeAnnotation>;
pub type TypeSlice = Box<TypeAnnotation>;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeArray {
    _type: Box<TypeAnnotation>,
    len: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeFnPtr {
    params: Vec<TypeAnnotation>,
    ret_type: Box<TypeAnnotation>,
}

impl TypeFnPtr {
    pub fn new(params: Vec<TypeAnnotation>, ret_type: TypeAnnotation) -> Self {
        TypeFnPtr {
            params,
            ret_type: Box::new(ret_type),
        }
    }

    pub fn from_item(item: &ItemFn) -> Self {
        TypeFnPtr::new(item.fn_params.type_annotations(), item.ret_type.clone())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct TypePtr {
    pub ptr_kind: PtrKind,
    pub _type: Box<TypeAnnotation>,
}

#[derive(StrEnum, PartialEq, Debug, Clone, Copy)]
pub enum TypeLitNum {
    F32,
    F64,
    #[strenum(disabled)]
    F,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    #[strenum(disabled)]
    I,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}
