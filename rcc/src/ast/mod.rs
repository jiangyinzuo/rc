use crate::lexer::token::Token;

pub mod file;
pub mod item;
pub mod types;
pub mod expr;
pub mod stmt;
pub mod pattern;

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Pub,
    Priv,
}

pub trait NamedASTNode {
    fn ident_name(&self) -> &str;
}

pub trait TokenStart {
    fn is_token_start(tk: &Token) -> bool;
}