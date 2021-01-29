use crate::ast::expr::Expr;

pub trait Visit : Sized {
    fn visit_expr(&mut self, expr: Expr);
}