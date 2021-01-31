use crate::ast::item::Item;
use crate::analyser::scope::Scope;
use crate::analyser::scope::BULITIN_SCOPE;
use std::fmt::{Debug, Formatter};

pub struct File {
    pub items: Vec<Item>,
    pub scope: Scope,
}

impl File {
    pub fn new() -> File {
        let mut file_scope = Scope::new();
        file_scope.set_father_as_builtin_scope();
        File {
            items: vec![],
            scope: file_scope
        }
    }

    pub fn items(mut self, items: Vec<Item>) -> File {
        self.items = items;
        self
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.items.eq(&other.items)
    }
}