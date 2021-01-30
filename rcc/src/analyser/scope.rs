use crate::analyser::sym_resolver::{TypeInfo, VarInfo};
use crate::ast::item::{ItemFn, ItemStruct};
use std::collections::HashMap;

pub struct Scope {
    father: Option<NonNull<Scope>>,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, VarInfo>,
}

use crate::analyser::sym_resolver::TypeInfo::*;
use lazy_static::lazy_static;
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
        }
    }

    pub fn add_type_fn(&mut self, item_fn: &ItemFn) {
        let type_info = TypeInfo::from_item_fn(item_fn);
        self.types.insert(item_fn.name.clone(), type_info);
    }

    pub fn add_type_struct(&mut self, item_struct: &ItemStruct) {
        let type_info = TypeInfo::from_item_struct(item_struct);
        self.types.insert(item_struct.name().to_string(), type_info);
    }

    pub fn set_father(&mut self, father: &Scope) {
        self.father = Some(NonNull::from(father));
    }

    pub fn set_father_from_non_null(&mut self, father: NonNull<Scope>) {
        self.father = Some(father);
    }
}
