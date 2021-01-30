use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::item::Item;


pub trait Visit : Sized {
    fn visit_file(&mut self, file: &File);

    fn visit_item(&mut self, item: &Item);

    fn visit_expr(&mut self, expr: &Expr);
}