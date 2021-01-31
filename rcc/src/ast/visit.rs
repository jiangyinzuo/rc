use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr,
    FieldAccessExpr, GroupedExpr, IfExpr, LitExpr, LoopExpr, PathExpr, RangeExpr, ReturnExpr,
    StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::rcc::RccError;

pub trait Visit: Sized {
    fn visit_file(&mut self, file: &mut File) -> Result<(), RccError>;

    fn visit_item(&mut self, item: &mut Item) -> Result<(), RccError>;

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError>;

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError>;

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RccError>;

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError>;

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError>;

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError>;

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<(), RccError>;

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<(), RccError>;

    fn visit_lit_expr(&mut self, lit_expr: &mut LitExpr) -> Result<(), RccError>;

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<(), RccError>;

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) -> Result<(), RccError>;

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<(), RccError>;

    fn visit_range_expr(&mut self, range_expr: &mut RangeExpr) -> Result<(), RccError>;

    fn visit_bin_op_expr(&mut self, bin_op_expr: &mut BinOpExpr) -> Result<(), RccError>;

    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr) -> Result<(), RccError>;

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr) -> Result<(), RccError>;

    fn visit_array_index_expr(
        &mut self,
        array_index_expr: &mut ArrayIndexExpr,
    ) -> Result<(), RccError>;

    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr) -> Result<(), RccError>;

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<(), RccError>;

    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr) -> Result<(), RccError>;

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<(), RccError>;

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<(), RccError>;

    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr) -> Result<(), RccError>;

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<(), RccError>;

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<(), RccError>;

    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr) -> Result<(), RccError>;

    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr) -> Result<(), RccError>;
}
