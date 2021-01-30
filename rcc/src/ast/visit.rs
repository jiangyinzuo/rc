use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::item::Item;


pub trait Visit : Sized {
    fn visit_file(&mut self, file: &File) {
        for item in file.items.iter() {
            self.visit_item(item);
        }
    }

    fn visit_item(&mut self, inner_item: &Item);

    fn visit_expr(&mut self, expr: &Expr);
}