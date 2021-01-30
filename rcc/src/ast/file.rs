use crate::ast::item::Item;

#[derive(Debug, PartialEq)]
pub struct File {
    pub items: Vec<Item>,
}
