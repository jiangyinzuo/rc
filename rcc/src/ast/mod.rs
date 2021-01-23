pub mod file;
pub mod item;
pub mod types;
pub mod expr;

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Pub,
    Priv,
}

pub trait NamedASTNode {
    fn ident_name(&self) -> &str;
}
