use super::pattern::Pattern;
use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::{Expr, ExprVisit};
use crate::ast::item::Item;
use crate::ast::stmt::Stmt::ExprStmt;
use crate::ast::types::TypeAnnotation;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Semi,
    Item(Item),
    Let(LetStmt),
    ExprStmt(Expr),
}

impl Stmt {
    pub fn type_info(&self) -> TypeInfo {
        match self {
            Self::Semi | Self::Item(_) | Self::Let(_) => TypeInfo::Unit,
            Self::ExprStmt(e) => {
                if e.with_block() {
                    let tp = e.type_info();
                    let t = tp.borrow();
                    t.deref().clone()
                } else if let Expr::Return(_) = e {
                    TypeInfo::Never
                } else {
                    TypeInfo::Unit
                }
            }
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        ExprStmt(expr)
    }
}

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub pattern: Pattern,
    pub _type: Option<TypeAnnotation>,
    pub rhs: Option<Expr>,
}

impl LetStmt {
    pub fn new(pattern: Pattern) -> Self {
        LetStmt {
            pattern,
            _type: None,
            rhs: None,
        }
    }

    pub fn _type(mut self, _type: TypeAnnotation) -> Self {
        self._type = Some(_type);
        self
    }

    pub fn expr(mut self, expr: Expr) -> Self {
        self.rhs = Some(expr);
        self
    }

    pub fn is_mut(&self) -> bool {
        match &self.pattern {
            Pattern::Identifier(i) => i.is_mut(),
        }
    }
}
