use crate::analyser::sym_resolver::TypeInfo::*;
use crate::analyser::sym_resolver::{TypeInfo, VarInfo, VarKind};
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::types::TypeLitNum::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Deref;
use std::ptr::NonNull;

lazy_static! {
    pub static ref BULITIN_SCOPE: Scope = {
        let mut s = Scope::new();
        s.types.insert("bool".into(), Bool);
        s.types.insert("char".into(), Char);
        s.types.insert("str".into(), Str);
        s.types.insert("f32".into(), LitNum(F32));
        s.types.insert("f64".into(), LitNum(F64));
        s.types.insert("i8".into(), LitNum(I8));
        s.types.insert("i16".into(), LitNum(I16));
        s.types.insert("i32".into(), LitNum(I32));
        s.types.insert("i64".into(), LitNum(I64));
        s.types.insert("i128".into(), LitNum(I128));
        s.types.insert("isize".into(), LitNum(Isize));
        s.types.insert("u8".into(), LitNum(U8));
        s.types.insert("u16".into(), LitNum(U16));
        s.types.insert("u32".into(), LitNum(U32));
        s.types.insert("u64".into(), LitNum(U64));
        s.types.insert("u128".into(), LitNum(U128));
        s.types.insert("usize".into(), LitNum(Usize));
        s
    };
}

pub struct Scope {
    father: Option<NonNull<Scope>>,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, Vec<VarInfo>>,
    pub cur_stmt_id: u64,
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

    pub fn add_variable(&mut self, ident: &str, kind: VarKind, type_info: TypeInfo) {
        let var_info = VarInfo::new(self.cur_stmt_id, kind, type_info);
        if let Some(v) = self.variables.get_mut(ident) {
            v.push(var_info);
        } else {
            self.variables.insert(ident.to_string(), vec![var_info]);
        }
    }

    pub fn find_variable(&mut self, ident: &str) -> Option<&VarInfo> {
        if let Some(v) = self.variables.get(ident) {
            let mut left = 0;
            let mut right = v.len();
            if right == 1 {
                return Some(unsafe {v.get_unchecked(0)});
            }
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

    pub fn find_def_except_fn(&self, ident: &str) -> TypeInfo {
        let mut cur_scope: *const Scope = self;
        loop {
            let s = unsafe { &*cur_scope };
            if let Some(ti) = s.types.get(ident) {
                if let TypeInfo::Fn { .. } = ti {
                } else {
                    return ti.clone();
                }
            }
            if let Some(f) = s.father {
                cur_scope = f.as_ptr();
            } else {
                return Unknown;
            }
        }
    }

    pub fn find_fn(&mut self, ident: &str) -> TypeInfo {
        let mut cur_scope: *const Scope = self;
        loop {
            let s = unsafe { &*cur_scope };
            if let Some(ti) = s.types.get(ident) {
                if let TypeInfo::Fn { .. } = ti {
                    return ti.clone();
                }
            }
            if let Some(f) = s.father {
                cur_scope = f.as_ptr();
            } else {
                return Unknown;
            }
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
        self.father = Some(unsafe { NonNull::new_unchecked(father) });
    }

    pub fn set_father_as_builtin_scope(&mut self) {
        self.father = Some(NonNull::from(BULITIN_SCOPE.deref()));
    }
}
