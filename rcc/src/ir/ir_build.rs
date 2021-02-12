use crate::analyser::scope::ScopeStack;
use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::{ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr, ExprKind, ExprVisit, FieldAccessExpr, GroupedExpr, IfExpr, LhsExpr, LitNumExpr, LoopExpr, PathExpr, RangeExpr, ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, UnOp, WhileExpr, BinOperator};
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::{TypeAnnotation, TypeLitNum};
use crate::ast::AST;
use crate::ir::{Func, IRInst, IRType, Operand, Place, IR};
use crate::rcc::RccError;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct IRBuilder {
    ir_output: IR,
    cur_fn: Func,

    scope_stack: ScopeStack,
}

impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder {
            ir_output: IR::new(),
            cur_fn: Func::new(),
            scope_stack: ScopeStack::new(),
        }
    }

    pub(super) fn generate_ir(&mut self, ast: &mut AST) -> Result<IR, RccError> {
        self.visit_file(&mut ast.file)?;
        let mut output = IR::new();
        std::mem::swap(&mut self.ir_output, &mut output);
        Ok(output)
    }

    fn gen_temp_variable(&mut self, type_info: Rc<RefCell<TypeInfo>>) -> Place {
        Place::Var(
            self.scope_stack
                .cur_scope_mut()
                .gen_temp_variable(type_info),
        )
    }

    fn gen_variable(&mut self, ident: &str) -> Place {
        Place::Var(format!(
            "{}_{}",
            ident,
            self.scope_stack.cur_scope().scope_id()
        ))
    }

    fn visit_file(&mut self, file: &mut File) -> Result<(), RccError> {
        self.scope_stack.enter_file(file);
        for item in file.items.iter_mut() {
            self.visit_item(item)?;
        }
        Ok(())
    }

    fn visit_item(&mut self, item: &mut Item) -> Result<(), RccError> {
        match item {
            Item::Fn(item_fn) => self.visit_item_fn(item_fn),
            Item::Struct(item_struct) => self.visit_item_struct(item_struct),
            _ => unimplemented!(),
        }
    }

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError> {
        let info = self.scope_stack.cur_scope().find_fn(&item_fn.name);
        assert_eq!(info, TypeInfo::from_item_fn(item_fn));

        let dest = self.gen_temp_variable(item_fn.fn_block.type_info());
        let operand = self.visit_block_expr(&mut item_fn.fn_block, dest)?;

        // add ret
        self.ir_output.add_instructions(IRInst::Ret(operand));

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
            Stmt::ExprStmt(expr) => {
                let dest = self.gen_temp_variable(expr.type_info());
                self.visit_expr(expr, dest);
                Ok(())
            }
        }
    }

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError> {
        if let Some(expr) = &mut let_stmt.expr {
            match &let_stmt.pattern {
                Pattern::Identifier(ident_pattern) => {
                    let ident = ident_pattern.ident();
                    let dest = self.gen_variable(ident);
                    self.visit_expr(expr, dest)?;
                }
            }
        }
        Ok(())
    }

    fn visit_expr(&mut self, expr: &mut Expr, dest: Place) -> Result<Operand, RccError> {
        let result = match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::LitNum(lit_num_expr) => self.visit_lit_num_expr(lit_num_expr),
            Expr::LitBool(lit_bool) => self.visit_lit_bool(lit_bool),
            Expr::LitChar(lit_char) => self.visit_lit_char(lit_char),
            Expr::LitStr(s) => self.visit_lit_str(s),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Block(block_expr) => self.visit_block_expr(block_expr, dest),
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            // Expr::Range(range_expr) => self.visit_range_expr(range_expr),
            Expr::BinOp(bin_op_expr) => self.visit_bin_op_expr(bin_op_expr, dest),
            Expr::Grouped(grouped_expr) => self.visit_grouped_expr(grouped_expr, dest),
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

    fn visit_lhs_expr(&mut self, lhs_expr: &mut LhsExpr) -> Result<Operand, RccError> {
        let r = match lhs_expr {
            LhsExpr::Path(expr) => self.visit_path_expr(expr)?,
            _ => todo!("visit lhs expr"),
        };
        Ok(r)
    }

    fn visit_grouped_expr(
        &mut self,
        grouped_expr: &mut GroupedExpr,
        dest: Place,
    ) -> Result<Operand, RccError> {
        self.visit_expr(grouped_expr, dest)
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_ident_pattern(
        &mut self,
        ident_pattern: &mut IdentPattern,
    ) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<Operand, RccError> {
        // TODO path segmentation
        Ok(Operand::Place(
            self.gen_variable(path_expr.segments.last().unwrap()),
        ))
    }

    fn visit_lit_num_expr(&mut self, lit_num_expr: &mut LitNumExpr) -> Result<Operand, RccError> {
        let t = lit_num_expr.get_lit_type();
        Ok(match t {
            TypeLitNum::I8 => Operand::I8(lit_num_expr.value.parse()?),
            TypeLitNum::I16 => Operand::I16(lit_num_expr.value.parse()?),
            TypeLitNum::I | TypeLitNum::I32 => Operand::I32(lit_num_expr.value.parse()?),
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
            TypeLitNum::F | TypeLitNum::F64 => Operand::F64(lit_num_expr.value.parse()?),
        })
    }

    fn visit_lit_bool(&mut self, lit_bool: &mut bool) -> Result<Operand, RccError> {
        Ok(Operand::Bool(*lit_bool))
    }

    fn visit_lit_char(&mut self, lit_char: &mut char) -> Result<Operand, RccError> {
        Ok(Operand::Char(*lit_char))
    }

    fn visit_lit_str(&mut self, s: &String) -> Result<Operand, RccError> {
        Ok(self.ir_output.add_lit_str(s.to_string()))
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<Operand, RccError> {
        let d = self.gen_temp_variable(unary_expr.expr.type_info());
        let operand = self.visit_expr(&mut unary_expr.expr, d)?;
        todo!()
    }

    fn visit_block_expr(
        &mut self,
        block_expr: &mut BlockExpr,
        dest: Place,
    ) -> Result<Operand, RccError> {
        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt)?;
        }

        Ok(if let Some(expr) = &mut block_expr.expr_without_block {
            self.visit_expr(&mut *expr, dest)?
        } else {
            Operand::Unit
        })
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<Operand, RccError> {
        let operand = self.visit_lhs_expr(&mut assign_expr.lhs)?;
        let p = match operand {
            Operand::Place(p) => p,
            _ => unimplemented!(),
        };
        let rhs = self.visit_expr(&mut assign_expr.rhs, p)?;
        Ok(Operand::Unit)
    }

    fn visit_range_expr(&mut self, range_expr: &mut RangeExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_bin_op_expr(
        &mut self,
        bin_op_expr: &mut BinOpExpr,
        dest: Place,
    ) -> Result<Operand, RccError> {
        let d = self.gen_temp_variable(bin_op_expr.lhs.type_info());
        let lhs = self.visit_expr(&mut bin_op_expr.lhs, d)?;
        let d = self.gen_temp_variable(bin_op_expr.rhs.type_info());
        let rhs = self.visit_expr(&mut bin_op_expr.rhs, d)?;

        // TODO operator override
        let _type = bin_op_expr.type_info();
        let t = _type.borrow();
        let tp = t.deref();
        let ir_type = IRType::from_type_info(tp)?;

        self.ir_output.add_instructions(IRInst::bin_op(
            bin_op_expr.bin_op,
            ir_type,
            dest.clone(),
            lhs,
            rhs,
        ));
        Ok(Operand::Place(dest))
    }

    /// ## Example1
    ///
    /// let a = foo() && bar();
    ///
    /// ...
    /// a_0 = foo()
    /// if not a_0 jump LABEL1
    /// $0 = bar()
    /// a_0 = $0
    /// LABEL1:
    /// ...
    ///
    /// ## Example2
    ///
    /// if foo() && bar() {
    ///     ...
    /// }
    ///
    /// ...
    /// $0_0 = foo()
    /// if not a_0 jump LABEL1
    /// $1_0 = bar()
    ///
    fn visit_and_and_expr(
        &mut self,
        bin_op_expr: &mut BinOpExpr,
        dest: Place,
    ) -> Result<Operand, RccError> {
        debug_assert_eq!(bin_op_expr.bin_op, BinOperator::AndAnd);
        todo!()
    }

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_array_index_expr(
        &mut self,
        array_index_expr: &mut ArrayIndexExpr,
    ) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<Operand, RccError> {
        for cond in if_expr.conditions.iter_mut() {}
        unimplemented!()
    }

    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }

    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr) -> Result<Operand, RccError> {
        unimplemented!()
    }
}
