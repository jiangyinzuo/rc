use std::collections::HashMap;
use crate::analyser::sym_resolver::{TypeInfo, VarInfo};

pub struct Scope {
    father: *const Scope,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, VarInfo>,
}

impl Scope {
    pub fn new(father: *const Scope) -> Self {
        Scope {
            father,
            types: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}
