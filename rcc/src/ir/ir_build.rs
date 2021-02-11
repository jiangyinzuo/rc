use crate::analyser::scope::ScopeStack;
use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, ExprVisit,
    FieldAccessExpr, IfExpr, LhsExpr, LitNumExpr, LoopExpr, PathExpr, RangeExpr,
    ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::item::{ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::TypeLitNum;
use crate::ast::visit::Visit;
use crate::ast::AST;
use crate::ir::{Func, IRInst, IRType, Operand, Place, IR};
use crate::rcc::RccError;
use std::ops::Deref;

pub struct IRBuilder {
    ir_output: IR,
    cur_fn: Func,

    scope_stack: ScopeStack,

    last_operand: Operand,
}

impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder {
            ir_output: IR::new(),
            cur_fn: Func::new(),
            scope_stack: ScopeStack::new(),
            last_operand: Operand::Bool(false),
        }
    }

    pub(super) fn generate_ir(&mut self, ast: &mut AST) -> Result<IR, RccError> {
        self.visit_file(&mut ast.file)?;
        let mut output = IR::new();
        std::mem::swap(&mut self.ir_output, &mut output);
        Ok(output)
    }
}

impl Visit for IRBuilder {
    type ReturnType = ();

    fn scope_stack_mut(&mut self) -> &mut ScopeStack {
        &mut self.scope_stack
    }

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError> {
        let info = self.scope_stack.cur_scope().find_fn(&item_fn.name);
        assert_eq!(info, TypeInfo::from_item_fn(item_fn));

        // push current func to IR
        let mut cur_fn = Func::new();
        std::mem::swap(&mut cur_fn, &mut self.cur_fn);
        self.ir_output.add_func(cur_fn);
        Ok(())
    }

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RccError> {
        match stmt {
            Stmt::Semi => Ok(()),
            Stmt::Item(item) => self.visit_item(item),
            Stmt::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Stmt::ExprStmt(expr) => self.visit_expr(expr),
        }
    }

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError> {
        if let Some(expr) = &mut let_stmt.expr {
            self.visit_expr(expr)?;
            let rhs = self.last_operand.clone();
            match &let_stmt.pattern {
                Pattern::Identifier(ident_pattern) => {
                    let ident = ident_pattern.ident();
                    let scope_id = self.scope_stack.cur_scope().scope_id();
                    self.ir_output
                        .add_instructions(IRInst::load_data(Place::var(ident, scope_id), rhs));
                }
            }
        }
        Ok(())
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError> {
        unimplemented!()
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<Self::ReturnType, RccError> {
        // TODO path segmentation
        self.last_operand = Operand::Place(Place::Var(path_expr.segments.last().unwrap().clone()));
        Ok(())
    }

    fn visit_lit_num_expr(
        &mut self,
        lit_num_expr: &mut LitNumExpr,
    ) -> Result<Self::ReturnType, RccError> {
        let t = lit_num_expr.get_lit_type();
        self.last_operand = match t {
            TypeLitNum::I8 => Operand::I8(lit_num_expr.value.parse()?),
            TypeLitNum::I16 => Operand::I16(lit_num_expr.value.parse()?),
            TypeLitNum::I32 => Operand::I32(lit_num_expr.value.parse()?),
            TypeLitNum::I64 => Operand::I64(lit_num_expr.value.parse()?),
            TypeLitNum::I128 => Operand::I128(lit_num_expr.value.parse()?),
            TypeLitNum::Isize => Operand::Isize(lit_num_expr.value.parse()?),
            TypeLitNum::U8 => Operand::U8(lit_num_expr.value.parse()?),
            TypeLitNum::U16 => Operand::U16(lit_num_expr.value.parse()?),
            TypeLitNum::U32 => Operand::U32(lit_num_expr.value.parse()?),
            TypeLitNum::U64 => Operand::U64(lit_num_expr.value.parse()?),
            TypeLitNum::U128 => Operand::U128(lit_num_expr.value.parse()?),
            TypeLitNum::Usize => Operand::Usize(lit_num_expr.value.parse()?),
            TypeLitNum::F32 => Operand::F32(lit_num_expr.value.parse()?),
            TypeLitNum::F64 => Operand::F64(lit_num_expr.value.parse()?),
            TypeLitNum::F | TypeLitNum::I => return Err("uncertain lit num type".into()),
        };
        Ok(())
    }

    fn visit_lit_bool(&mut self, lit_bool: &mut bool) -> Result<Self::ReturnType, RccError> {
        self.last_operand = Operand::Bool(*lit_bool);
        Ok(())
    }

    fn visit_lit_char(&mut self, lit_char: &mut char) -> Result<Self::ReturnType, RccError> {
        self.last_operand = Operand::Char(*lit_char);
        Ok(())
    }

    fn visit_lit_str(&mut self, s: &String) -> Result<Self::ReturnType, RccError> {
        self.last_operand = self.ir_output.add_lit_str(s.to_string());
        Ok(())
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
        self.visit_expr(&mut assign_expr.rhs)?;
        let rhs = self.last_operand.clone();
        self.visit_lhs_expr(&mut assign_expr.lhs)?;
        let lhs = self.last_operand.clone();
        let p = match lhs {
            Operand::Place(p) => p,
            _ => unreachable!(),
        };
        self.ir_output.add_instructions(IRInst::load_data(p, rhs));
        Ok(())
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
        self.visit_expr(&mut bin_op_expr.lhs)?;
        let lhs = self.last_operand.clone();
        self.visit_expr(&mut bin_op_expr.rhs)?;
        let rhs = self.last_operand.clone();
        // TODO operator override
        let _type = bin_op_expr.type_info();
        let t = _type.borrow();
        let tp = t.deref();
        let ir_type = IRType::from_type_info(tp)?;
        let scope = self.scope_stack.cur_scope_mut();
        let ident = scope.gen_temp_variable(_type.clone());

        self.ir_output.add_instructions(IRInst::bin_op(
            bin_op_expr.bin_op,
            ir_type,
            Place::Var(ident.clone()),
            lhs,
            rhs,
        ));
        self.last_operand = Operand::Place(Place::Var(ident));
        Ok(())
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
