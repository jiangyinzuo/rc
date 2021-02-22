use crate::ast::expr::BlockExpr;
use crate::ast::pattern::Pattern;
use crate::ast::types::TypeAnnotation;
use crate::ast::{NamedASTNode, TokenStart, Visibility};
use crate::lexer::token::Token;
use crate::rcc::RccError;

#[derive(Debug, PartialEq)]
pub enum Item {
    /// fn add(a, b) { a + b }
    Fn(ItemFn),

    /// struct Foo { x: i32 }
    Struct(ItemStruct),

    /// enum Color { Red, Yellow }
    Enum(TypeEnum),

    /// type Int = i32;
    Type,

    /// const A: i32 = 2;
    Const,

    /// static B: i32 = 3;
    Static,

    /// impl Foo { ... }
    Impl,

    /// extern "C" {}
    ExternalBlock(ItemExternalBlock),
}

impl TokenStart for Item {
    fn is_token_start(tk: &Token) -> bool {
        matches!(
            tk,
            Token::Pub
                | Token::Priv
                | Token::Fn
                | Token::Const
                | Token::Static
                | Token::Struct
                | Token::Enum
                | Token::Impl
        )
    }
}

impl NamedASTNode for Item {
    fn ident_name(&self) -> &str {
        match self {
            Self::Fn(item_fn) => item_fn.ident_name(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ItemFn {
    vis: Visibility,
    pub name: String,
    pub fn_params: FnParams,
    pub ret_type: TypeAnnotation,
    pub fn_block: BlockExpr,
}

impl ItemFn {
    pub fn new(
        vis: Visibility,
        name: String,
        fn_params: FnParams,
        ret_type: TypeAnnotation,
        fn_block: BlockExpr,
    ) -> Self {
        ItemFn {
            vis,
            name,
            fn_params,
            ret_type,
            fn_block,
        }
    }

    pub fn vis(&self) -> Visibility {
        self.vis.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct FnParams {
    pub params: Vec<FnParam>,
}

impl FnParams {
    pub fn new() -> FnParams {
        FnParams { params: vec![] }
    }

    pub fn type_annotations(&self) -> Vec<TypeAnnotation> {
        self.params
            .iter()
            .map(|param| param._type.clone())
            .collect()
    }

    pub fn push(&mut self, param: FnParam) {
        self.params.push(param);
    }
}

impl From<Vec<FnParam>> for FnParams {
    fn from(params: Vec<FnParam>) -> Self {
        FnParams { params }
    }
}

#[derive(Debug, PartialEq)]
pub struct FnParam {
    pub pattern: Pattern,
    pub _type: TypeAnnotation,
}

impl FnParam {
    pub fn new(pattern: Pattern, _type: TypeAnnotation) -> Self {
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

/// # Examples
/// `struct Student { name: String, age: u32 }`
/// `pub struct Teacher(String, u32);`
#[derive(Debug, PartialEq)]
pub struct ItemStruct {
    vis: Visibility,
    name: String,
    fields: Fields,
}

impl ItemStruct {
    pub fn new(vis: Visibility, name: String) -> Self {
        ItemStruct {
            vis,
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

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn vis(&self) -> Visibility {
        self.vis.clone()
    }
}

/// enum Identity {
///     Student { name: String },
///     Teacher(String),
///     Admin,
/// }
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TypeEnum {
    vis: Visibility,
    name: String,
    enum_items: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct EnumVariant {
    name: String,
    fields: Fields,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Fields {
    /// `struct Foo {a: i32}`
    Struct(Vec<StructField>),
    /// `struct Foo(i32);`
    Tuple(Vec<TupleField>),
    /// `struct Foo;`
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StructField {
    pub vis: Visibility,
    pub name: String,
    pub _type: TypeAnnotation,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleField {
    pub vis: Visibility,
    pub _type: TypeAnnotation,
}

/// `extern "C" { fn foo(); }`
#[derive(Debug, PartialEq)]
pub struct ItemExternalBlock {
    abi: ABI,
    external_items: Vec<ExternalItem>,
}

impl ItemExternalBlock {
    pub fn new(abi: ABI, external_items: Vec<ExternalItem>) -> ItemExternalBlock {
        ItemExternalBlock {
            abi,
            external_items,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ABI {
    C,
}

impl ABI {
    pub fn from_string(s: &str) -> Result<ABI, RccError> {
        match s {
            "C" => Ok(ABI::C),
            _ => Err(format!("invalid abi {}", s).into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExternalItem {
    Fn(ExternalItemFn),
}

#[derive(Debug, PartialEq)]
pub struct ExternalItemFn {
    vis: Visibility,
    pub name: String,
    pub fn_params: FnParams,
    pub ret_type: TypeAnnotation,
}

impl ExternalItemFn {
    pub fn new(
        vis: Visibility,
        name: String,
        fn_params: FnParams,
        ret_type: TypeAnnotation,
    ) -> ExternalItemFn {
        ExternalItemFn {
            vis,
            name,
            fn_params,
            ret_type,
        }
    }
}
