use crate::ast::item::ItemFn;
use crate::ast::types::TypeAnnotation::{Identifier, Tuple};
use std::fmt::{Debug, Formatter};
use strenum::StrEnum;

#[derive(PartialEq, Clone, Eq, Hash)]
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
    Unit,

    Bool,
    Str,
    Char,
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
            Self::Unit => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Str => write!(f, "&str"),
            Self::Char => write!(f, "char"),
            Self::Unknown => write!(f, "[unknown]"),
        }
    }
}

pub type TypeTuple = Vec<TypeAnnotation>;
pub type TypeSlice = Box<TypeAnnotation>;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct TypeArray {
    _type: Box<TypeAnnotation>,
    len: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TypeFnPtr {
    pub params: Vec<TypeAnnotation>,
    pub ret_type: Box<TypeAnnotation>,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PtrKind {
    /// &i32
    Ref,
    /// &mut i32
    MutRef,
    /// *mut i32
    MutRawPtr,
    /// *const i32
    ConstRawPtr,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TypePtr {
    pub ptr_kind: PtrKind,
    pub type_anno: Box<TypeAnnotation>,
}

impl TypePtr {
    pub fn new(ptr_kind: PtrKind, type_anno: TypeAnnotation) -> TypePtr {
        TypePtr {
            ptr_kind,
            type_anno: Box::new(type_anno),
        }
    }
}

#[derive(StrEnum, PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

impl TypeLitNum {
    pub fn is_integer(&self) -> bool {
        use TypeLitNum::*;
        matches!(
            self,
            I8 | I16 | I32 | I64 | I128 | Isize | I | U8 | U16 | U32 | U64 | U128 | Usize
        )
    }

    pub fn is_float(&self) -> bool {
        use TypeLitNum::*;
        matches!(self, F | F32 | F64)
    }
}
