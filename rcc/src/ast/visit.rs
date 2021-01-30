use crate::ast::expr::{BlockExpr, Expr, IfExpr, UnAryExpr, AssignExpr, RangeExpr, BinOpExpr, GroupedExpr, ArrayExpr, ArrayIndexExpr, TupleIndexExpr, StructExpr, CallExpr, FieldAccessExpr, WhileExpr, LoopExpr, ReturnExpr, BreakExpr, TupleExpr};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::stmt::Stmt;

pub trait Visit: Sized {
    fn visit_file(&mut self, file: &mut File);

    fn visit_item(&mut self, item: &mut Item);

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn);

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct);

    fn visit_stmt(&mut self, stmt: &mut Stmt);

    fn visit_expr(&mut self, expr: &mut Expr);

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr);
    
    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr);

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr);

    fn visit_range_expr(&mut self, range_expr: &mut RangeExpr);
    
    fn visit_bin_op_expr(&mut self, bin_op_expr: &mut BinOpExpr);
    
    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr);
    
    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr);
    
    fn visit_array_index_expr(&mut self, array_index_expr: &mut ArrayIndexExpr);
    
    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr);
    
    fn visit_tuple_index_expr(&mut self, tuple_index_expr: &mut TupleIndexExpr);
    
    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr);
    
    fn visit_call_expr(&mut self, call_expr: &mut CallExpr);
    
    fn visit_field_access_expr(&mut self, field_access_expr: &mut FieldAccessExpr);
    
    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr);
    
    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr);
    
    fn visit_if_expr(&mut self, if_expr: &mut IfExpr);
    
    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr);
    
    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr);
}
