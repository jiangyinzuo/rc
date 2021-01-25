use super::pattern::Pattern;
use crate::ast::expr::Expr;
use crate::ast::item::VisItem;
use crate::ast::types::Type;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Semi,
    Item(VisItem),
    Let(LetStmt),
    ExprStmt(Expr),
}

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pattern: Pattern,
    _type: Type,
    expr: Expr,
}
