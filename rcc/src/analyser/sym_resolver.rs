use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemFn};
use crate::ast::types::{TypeAnnotation, TypeFnPtr};
use crate::ast::visit::Visit;
use crate::ast::Visibility;
use std::ptr::NonNull;
use crate::analyser::scope::Scope;

pub enum VarKind {
    Static,
    Const,
    LocalMut,
    Local,
}

pub struct VarInfo {
    kind: VarKind,
    _type: TypeAnnotation,
}

pub enum TypeInfo {
    Fn { vis: Visibility, inner: TypeFnPtr },
    Struct { vis: Visibility, fields: Fields },
}

impl TypeInfo {
    fn from_item_fn(item: &ItemFn) -> Self {
        let tp_fn_ptr = TypeFnPtr::from_item(item);
        Self::Fn {
            vis: item.vis.clone(),
            inner: tp_fn_ptr,
        }
    }
}

pub struct SymbolResolver {
    /// global scope
    file_scope: Scope,
    cur_scope: NonNull<Scope>,
}

impl SymbolResolver {
    fn new() -> SymbolResolver {
        let mut file_scope = Scope::new(std::ptr::null());
        let cur_scope = NonNull::new(&mut file_scope).expect("&mut file_scope is null");
        SymbolResolver {
            file_scope,
            cur_scope,
        }
    }
}

impl Visit for SymbolResolver {
    fn visit_file(&mut self, file: &File) {
        for item in file.items.iter() {
            // match item {
            //     Item::Fn(item_fn) => {}
            // }
        }
    }

    fn visit_item(&mut self, item: &Item) {}

    fn visit_expr(&mut self, expr: &Expr) {
        pub fn bar() {}
    }
}

impl SymbolResolver {
    fn add_type_fn(&mut self, item_fn: &ItemFn) {
        let type_info = TypeInfo::from_item_fn(item_fn);
        unsafe {
            self.cur_scope
                .as_mut()
                .types
                .insert(item_fn.name.clone(), type_info);
        }
    }
}
