use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr,
    FieldAccessExpr, GroupedExpr, IfExpr, LhsExpr, LitNumExpr, LoopExpr, PathExpr, RangeExpr,
    ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::TypeLitNum;
use crate::ast::visit::Visit;
use crate::ast::AST;
use crate::ir::{IRType, Operand, IR};
use crate::rcc::RccError;

pub struct IRBuilder {
    ast: AST,
    ir_output: IR,
}

impl IRBuilder {
    pub fn new(ast: AST) -> IRBuilder {
        IRBuilder {
            ast,
            ir_output: IR::new(),
        }
    }

    pub(super) fn generate_ir(&mut self) -> IR {
        let mut output = IR::new();
        std::mem::swap(&mut self.ir_output, &mut output);
        output
    }
}

impl<'ast> Visit<'ast> for IRBuilder {
    type ReturnType = Operand;

    fn visit_file(&mut self, file: &'ast mut File) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_item(&mut self, item: &mut Item) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_lhs_expr(&mut self, lhs_expr: &mut LhsExpr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_lit_num_expr(&mut self, lit_num_expr: &mut LitNumExpr) -> Result<Self::ReturnType, RccError> {
        let t = lit_num_expr.get_lit_type();
        Ok(Operand::Imm(match t {
            TypeLitNum::I8 => IRType::I8(lit_num_expr.value.parse()?),
            TypeLitNum::I16 => IRType::I16(lit_num_expr.value.parse()?),
            TypeLitNum::I32 => IRType::I32(lit_num_expr.value.parse()?),
            TypeLitNum::I64 => IRType::I64(lit_num_expr.value.parse()?),
            TypeLitNum::I128 => IRType::I128(lit_num_expr.value.parse()?),
            TypeLitNum::Isize => IRType::Isize(lit_num_expr.value.parse()?),
            TypeLitNum::U8 => IRType::U8(lit_num_expr.value.parse()?),
            TypeLitNum::U16 => IRType::U16(lit_num_expr.value.parse()?),
            TypeLitNum::U32 => IRType::U32(lit_num_expr.value.parse()?),
            TypeLitNum::U64 => IRType::U64(lit_num_expr.value.parse()?),
            TypeLitNum::U128 => IRType::U128(lit_num_expr.value.parse()?),
            TypeLitNum::Usize => IRType::Usize(lit_num_expr.value.parse()?),
            TypeLitNum::F32 => IRType::F32(lit_num_expr.value.parse()?),
            TypeLitNum::F64 => IRType::F64(lit_num_expr.value.parse()?),
            TypeLitNum::F | TypeLitNum::I => return Err("uncertain lit num type".into()),
        }))
    }

    fn visit_lit_bool(&mut self, lit_bool: &mut bool) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_lit_char(&mut self, lit_char: &mut char) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_lit_str(&mut self, s: &String) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_unary_expr(
        &mut self,
        unary_expr: &mut UnAryExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_block_expr(
        &mut self,
        block_expr: &mut BlockExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_assign_expr(
        &mut self,
        assign_expr: &mut AssignExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_range_expr(
        &mut self,
        range_expr: &mut RangeExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_bin_op_expr(
        &mut self,
        bin_op_expr: &mut BinOpExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_grouped_expr(
        &mut self,
        grouped_expr: &mut GroupedExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_array_expr(
        &mut self,
        array_expr: &mut ArrayExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_array_index_expr(
        &mut self,
        array_index_expr: &mut ArrayIndexExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_tuple_expr(
        &mut self,
        tuple_expr: &mut TupleExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_struct_expr(
        &mut self,
        struct_expr: &mut StructExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_while_expr(
        &mut self,
        while_expr: &mut WhileExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_return_expr(
        &mut self,
        return_expr: &mut ReturnExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }

    fn visit_break_expr(
        &mut self,
        break_expr: &mut BreakExpr,
    ) -> Result<Self::ReturnType, RccError> {
        unimplemented!()
    }
}
