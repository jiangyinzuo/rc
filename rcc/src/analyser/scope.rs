use crate::analyser::sym_resolver::{TypeInfo, VarInfo};
use crate::ast::item::{ItemFn, ItemStruct};
use std::collections::HashMap;

pub struct Scope {
    father: *const Scope,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, VarInfo>,
}

impl Scope {
    pub fn new(father: *const Scope) -> Scope {
        Scope {
            father,
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
}
