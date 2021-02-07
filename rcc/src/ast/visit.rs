use crate::analyser::sym_resolver::{TypeInfo, VarInfo};
use crate::ast::expr::{ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr, FieldAccessExpr, GroupedExpr, IfExpr, LhsExpr, LitNumExpr, LoopExpr, PathExpr, RangeExpr, ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr, ExprKind, ExprVisit};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::rcc::RccError;

pub trait Visit<'ast>: Sized {
    type ReturnType;

    fn visit_file(&mut self, file: &'ast mut File) -> Result<(), RccError>;

    fn visit_item(&mut self, item: &mut Item) -> Result<(), RccError>;

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError>;

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError>;

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RccError>;

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError>;

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError>;

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError>;

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<Self::ReturnType, RccError> {
        let result = match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::LitNum(lit_num_expr) => self.visit_lit_num_expr(lit_num_expr),
            Expr::LitBool(lit_bool) => self.visit_lit_bool(lit_bool),
            Expr::LitChar(lig_char) => self.visit_lit_char(lig_char),
            Expr::LitStr(s) => self.visit_lit_str(s),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Block(block_expr) => self.visit_block_expr(block_expr),
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            // Expr::Range(range_expr) => self.visit_range_expr(range_expr),
            Expr::BinOp(bin_op_expr) => self.visit_bin_op_expr(bin_op_expr),
            Expr::Grouped(grouped_expr) => self.visit_grouped_expr(grouped_expr),
            // Expr::Array(array_expr) => self.visit_array_expr(array_expr),
            // Expr::ArrayIndex(array_index_expr) => self.visit_array_index_expr(array_index_expr),
            // Expr::Tuple(tuple_expr) => self.visit_tuple_expr(tuple_expr),
            // Expr::TupleIndex(tuple_index_expr) => self.visit_tuple_index_expr(tuple_index_expr),
            // Expr::Struct(struct_expr) => self.visit_struct_expr(struct_expr),
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            // Expr::FieldAccess(field_access_expr) => self.visit_field_access_expr(field_access_expr),
            Expr::While(while_expr) => self.visit_while_expr(while_expr),
            Expr::Loop(loop_expr) => self.visit_loop_expr(loop_expr),
            Expr::If(if_expr) => self.visit_if_expr(if_expr),
            Expr::Return(return_expr) => self.visit_return_expr(return_expr),
            Expr::Break(break_expr) => self.visit_break_expr(break_expr),
            _ => unimplemented!(),
        };
        debug_assert_ne!(
            ExprKind::Unknown,
            expr.kind(),
            "unknown expr kind: {:?}",
            expr
        );
        result
    }

    fn visit_lhs_expr(&mut self, lhs_expr: &mut LhsExpr) -> Result<Self::ReturnType, RccError>;

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<Self::ReturnType, RccError>;

    fn visit_lit_num_expr(
        &mut self,
        lit_num_expr: &mut LitNumExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_lit_bool(&mut self, lit_bool: &mut bool) -> Result<Self::ReturnType, RccError>;

    fn visit_lit_char(&mut self, lit_char: &mut char) -> Result<Self::ReturnType, RccError>;

    fn visit_lit_str(&mut self, s: &String) -> Result<Self::ReturnType, RccError>;

    fn visit_unary_expr(
        &mut self,
        unary_expr: &mut UnAryExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_block_expr(
        &mut self,
        block_expr: &mut BlockExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_assign_expr(
        &mut self,
        assign_expr: &mut AssignExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_range_expr(
        &mut self,
        range_expr: &mut RangeExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_bin_op_expr(
        &mut self,
        bin_op_expr: &mut BinOpExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_grouped_expr(
        &mut self,
        grouped_expr: &mut GroupedExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_array_expr(
        &mut self,
        array_expr: &mut ArrayExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_array_index_expr(
        &mut self,
        array_index_expr: &mut ArrayIndexExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_tuple_expr(
        &mut self,
        tuple_expr: &mut TupleExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_struct_expr(
        &mut self,
        struct_expr: &mut StructExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<Self::ReturnType, RccError>;

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_while_expr(
        &mut self,
        while_expr: &mut WhileExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<Self::ReturnType, RccError>;

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<Self::ReturnType, RccError>;

    fn visit_return_expr(
        &mut self,
        return_expr: &mut ReturnExpr,
    ) -> Result<Self::ReturnType, RccError>;

    fn visit_break_expr(
        &mut self,
        break_expr: &mut BreakExpr,
    ) -> Result<Self::ReturnType, RccError>;
}
