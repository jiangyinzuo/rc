use crate::ast::item::VisItem;

#[derive(Debug, PartialEq)]
pub struct File {
    pub items: Vec<VisItem>,
}
