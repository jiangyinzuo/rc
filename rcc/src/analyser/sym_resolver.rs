use crate::analyser::scope::Scope;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr,
    FieldAccessExpr, GroupedExpr, IfExpr, LitExpr, LoopExpr, PathExpr, RangeExpr, ReturnExpr,
    StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::{TypeAnnotation, TypeFnPtr};
use crate::ast::visit::Visit;
use crate::ast::Visibility;
use crate::rcc::RccError;
use std::ptr::NonNull;

#[derive(Debug, PartialEq)]
pub enum VarKind {
    Static,
    Const,
    LocalMut,
    Local,
}

#[derive(Debug, PartialEq)]
pub struct VarInfo {
    stmt_id: u64,
    kind: VarKind,
    _type: TypeAnnotation,
}

impl VarInfo {
    pub fn new(stmt_id: u64, kind: VarKind, _type: TypeAnnotation) -> VarInfo {
        VarInfo {
            stmt_id,
            kind,
            _type,
        }
    }

    pub fn stmt_id(&self) -> u64 {
        self.stmt_id
    }
}

pub enum TypeInfo {
    Fn {
        vis: Visibility,
        inner: TypeFnPtr,
    },
    Struct {
        vis: Visibility,
        fields: NonNull<Fields>,
    },

    /// primitive type
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl TypeInfo {
    pub(crate) fn from_item_fn(item: &ItemFn) -> Self {
        let tp_fn_ptr = TypeFnPtr::from_item(item);
        Self::Fn {
            vis: item.vis(),
            inner: tp_fn_ptr,
        }
    }

    pub(crate) fn from_item_struct(item: &ItemStruct) -> Self {
        Self::Struct {
            vis: item.vis(),
            fields: NonNull::from(item.fields()),
        }
    }
}

pub struct SymbolResolver {
    cur_scope: *mut Scope,
    file_scope: *mut Scope,
    scope_stack: Vec<*mut Scope>,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            cur_scope: std::ptr::null_mut(),
            file_scope: std::ptr::null_mut(),
            scope_stack: vec![],
        }
    }

    fn enter_block(&mut self, block_expr: &mut BlockExpr) {
        block_expr.scope.set_father(self.cur_scope);
        self.scope_stack.push(self.cur_scope);
        self.cur_scope = &mut block_expr.scope;
    }

    fn exit_block(&mut self) {
        if let Some(s) = self.scope_stack.pop() {
            self.cur_scope = s;
        } else {
            debug_assert!(false, "scope_stack is empty!");
        }
    }

    fn cur_scope_is_global(&mut self) -> bool {
        !self.file_scope.is_null() && self.cur_scope == self.file_scope
    }
}

impl Visit for SymbolResolver {
    fn visit_file(&mut self, file: &mut File) -> Result<(), RccError> {
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
        if let Some(block) = item_fn.fn_block.as_mut() {
            self.visit_block_expr(block)?;
        }
        Ok(())
    }

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError> {
        Ok(())
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
        }
        match &let_stmt.pattern {
            Pattern::Identifier(ident_pattern) => {}
        }
        Ok(())
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError> {
        Ok(())
    }

    /// visit the expression which may contain block expression
    fn visit_expr(&mut self, expr: &mut Expr) -> Result<(), RccError> {
        match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::Lit(lit_expr) => self.visit_lit_expr(lit_expr),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Block(block_expr) => self.visit_block_expr(block_expr),
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            Expr::Range(range_expr) => self.visit_range_expr(range_expr),
            Expr::BinOp(bin_op_expr) => self.visit_bin_op_expr(bin_op_expr),
            Expr::Grouped(grouped_expr) => self.visit_grouped_expr(grouped_expr),
            Expr::Array(array_expr) => self.visit_array_expr(array_expr),
            Expr::ArrayIndex(array_index_expr) => self.visit_array_index_expr(array_index_expr),
            Expr::Tuple(tuple_expr) => self.visit_tuple_expr(tuple_expr),
            Expr::TupleIndex(tuple_index_expr) => self.visit_tuple_index_expr(tuple_index_expr),
            Expr::Struct(struct_expr) => self.visit_struct_expr(struct_expr),
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            Expr::FieldAccess(field_access_expr) => self.visit_field_access_expr(field_access_expr),
            Expr::While(while_expr) => self.visit_while_expr(while_expr),
            Expr::Loop(loop_expr) => self.visit_loop_expr(loop_expr),
            Expr::If(if_expr) => self.visit_if_expr(if_expr),
            Expr::Return(return_expr) => self.visit_return_expr(return_expr),
            Expr::Break(break_expr) => self.visit_break_expr(break_expr),
            _ => Ok(()),
        }
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<(), RccError> {
        if let Some(ident) = path_expr.segments.last() {
            if let Some(var_info) = unsafe { (*self.cur_scope).find_variable(ident) } {
                return Ok(())
            } else {
                return Err(format!("identifier `{}` not found", ident).into())
            }
        } else {
            return Err("invalid ident".into());
        }
    }

    fn visit_lit_expr(&mut self, lit_expr: &mut LitExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<(), RccError> {
        self.visit_expr(&mut unary_expr.expr)
    }

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) -> Result<(), RccError> {
        self.enter_block(block_expr);
        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt)?;
        }
        if let Some(expr) = block_expr.expr_without_block.as_mut() {
            self.visit_expr(expr)?;
        }
        self.exit_block();
        Ok(())
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<(), RccError> {
        self.visit_expr(&mut assign_expr.lhs)?;
        self.visit_expr(&mut assign_expr.rhs)
    }

    fn visit_range_expr(&mut self, range_expr: &mut RangeExpr) -> Result<(), RccError> {
        if let Some(expr) = range_expr.lhs.as_mut() {
            self.visit_expr(expr)?;
        }
        if let Some(expr) = range_expr.rhs.as_mut() {
            self.visit_expr(expr)?;
        }
        Ok(())
    }

    fn visit_bin_op_expr(&mut self, bin_op_expr: &mut BinOpExpr) -> Result<(), RccError> {
        self.visit_expr(&mut bin_op_expr.lhs)?;
        self.visit_expr(&mut bin_op_expr.rhs)
    }

    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr) -> Result<(), RccError> {
        self.visit_expr(grouped_expr)
    }

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr) -> Result<(), RccError> {
        for e in array_expr.elems.iter_mut() {
            self.visit_expr(e)?;
        }
        if let Some(expr) = array_expr.len_expr.expr.as_mut() {
            self.visit_expr(expr)?;
        }
        Ok(())
    }

    fn visit_array_index_expr(
        &mut self,
        array_index_expr: &mut ArrayIndexExpr,
    ) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr) -> Result<(), RccError> {
        if let Some(expr) = return_expr.0.as_mut() {
            self.visit_expr(expr)?;
        }
        Ok(())
    }

    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr) -> Result<(), RccError> {
        if let Some(expr) = break_expr.0.as_mut() {
            self.visit_expr(expr)?;
        }
        Ok(())
    }
}
