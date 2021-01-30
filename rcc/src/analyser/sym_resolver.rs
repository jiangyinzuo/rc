use crate::ast::expr::Expr;
use crate::ast::item::{Item};
use crate::ast::types::Type;
use crate::ast::visit::Visit;
use crate::ast::{Visibility, NamedASTNode};
use std::collections::HashMap;
use std::ptr::NonNull;

pub enum VarKind {
    Static,
    Const,
    LocalMut,
    Local,
}

pub struct VarInfo {
    kind: VarKind,
    _type: Type,
}

pub struct TypeInfo {
    vis: Visibility,
    _type: Type,
}

pub struct Scope {
    father: Option<NonNull<Scope>>,
    types: HashMap<String, TypeInfo>,
    variables: HashMap<String, VarInfo>,
}

impl Scope {
    pub(super) fn new(father: *mut Scope) -> Self {
        Scope {
            father: NonNull::new(father),
            types: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

pub struct FileSymbolResolver {
    file_scope: Scope,
    cur_scope: NonNull<Scope>,
}

impl FileSymbolResolver {
    fn new() -> FileSymbolResolver {
        let mut file_scope = Scope::new(std::ptr::null_mut::<Scope>());
        let cur_scope = NonNull::new(&mut file_scope).expect("&mut file_scope is null");
        FileSymbolResolver {
            file_scope,
            cur_scope,
        }
    }
}

impl Visit for FileSymbolResolver {
    fn visit_item(&mut self, inner_item: &Item) {
        unimplemented!()
    }

    fn visit_expr(&mut self, expr: &Expr) {
        pub fn bar() {}
    }
}
