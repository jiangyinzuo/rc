use crate::analyser::sym_resolver::{TypeInfo, VarInfo};
use crate::ast::item::{Item, ItemFn, ItemStruct};
use std::collections::HashMap;

pub struct Scope {
    father: Option<NonNull<Scope>>,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, Vec<VarInfo>>,
    pub cur_stmt_id: u64,
}

use crate::analyser::sym_resolver::TypeInfo::*;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::ptr::NonNull;

lazy_static! {
    pub static ref BULITIN_SCOPE: Scope = {
        let mut s = Scope::new();
        s.types.insert("bool".into(), Bool);
        s.types.insert("char".into(), Char);
        s.types.insert("f32".into(), F32);
        s.types.insert("f64".into(), F64);
        s.types.insert("i8".into(), I8);
        s.types.insert("i16".into(), I16);
        s.types.insert("i32".into(), I32);
        s.types.insert("i64".into(), I64);
        s.types.insert("i128".into(), I128);
        s.types.insert("isize".into(), Isize);
        s.types.insert("u8".into(), U8);
        s.types.insert("u16".into(), U16);
        s.types.insert("u32".into(), U32);
        s.types.insert("u64".into(), U64);
        s.types.insert("u128".into(), U128);
        s.types.insert("usize".into(), Usize);
        s
    };
}

unsafe impl std::marker::Sync for Scope {}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            father: None,
            types: HashMap::new(),
            variables: HashMap::new(),
            cur_stmt_id: 0,
        }
    }

    pub fn add_variable(&mut self, ident: &str, info: VarInfo) {
        if let Some(v) = self.variables.get_mut(ident) {
            v.push(info);
        } else {
            self.variables.insert(ident.to_string(), vec![info]);
        }
    }

    pub fn find_variable(&mut self, ident: &str) -> Option<&VarInfo> {
        if let Some(v) = self.variables.get(ident) {
            let mut left = 0;
            let mut right = v.len();
            while left < right {
                let mid = (left + right + 1) / 2;
                let stmt_id = unsafe { (*v.get_unchecked(mid)).stmt_id() };
                // Let stmt and variable using stmt is impossible to be the same.
                debug_assert_ne!(stmt_id, self.cur_stmt_id);
                if self.cur_stmt_id < stmt_id {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            Some(unsafe { v.get_unchecked(left) })
        } else {
            None
        }
    }

    pub fn add_typedef(&mut self, item: &Item) {
        match item {
            Item::Fn(item_fn) => self.add_type_fn(item_fn),
            Item::Struct(item_struct) => self.add_type_struct(item_struct),
            _ => { /* TODO */ }
        }
    }

    fn add_type_fn(&mut self, item_fn: &ItemFn) {
        let type_info = TypeInfo::from_item_fn(item_fn);
        self.types.insert(item_fn.name.clone(), type_info);
    }

    fn add_type_struct(&mut self, item_struct: &ItemStruct) {
        let type_info = TypeInfo::from_item_struct(item_struct);
        self.types.insert(item_struct.name().to_string(), type_info);
    }

    pub fn set_father(&mut self, father: *mut Scope) {
        unsafe {
            self.father = Some(NonNull::new_unchecked(father));
        }
    }

    pub fn set_father_as_builtin_scope(&mut self) {
        self.father = Some(NonNull::from(BULITIN_SCOPE.deref()));
    }
}
