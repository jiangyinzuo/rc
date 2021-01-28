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
    _type: Option<Type>,
    expr: Option<Expr>,
}

impl LetStmt {
    pub fn new(pattern: Pattern) -> Self {
        LetStmt {
            pattern,
            _type: None,
            expr: None,
        }
    }

    pub fn _type(mut self, _type: Type) -> Self {
        self._type = Some(_type);
        self
    }

    pub fn expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }
}
