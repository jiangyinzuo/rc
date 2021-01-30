use super::scope::BULITIN_SCOPE;
use crate::analyser::scope::Scope;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr,
    FieldAccessExpr, GroupedExpr, IfExpr, LoopExpr, RangeExpr, ReturnExpr, StructExpr, TupleExpr,
    TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemFn, ItemStruct};
use crate::ast::stmt::Stmt;
use crate::ast::types::{TypeAnnotation, TypeFnPtr};
use crate::ast::visit::Visit;
use crate::ast::Visibility;
use std::ptr::NonNull;

pub enum VarKind {
    Static,
    Const,
    LocalMut,
    Local,
}

pub struct VarInfo {
    kind: VarKind,
    _type: TypeAnnotation,
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
    /// global scope
    file_scope: Scope,
    cur_scope: NonNull<Scope>,
    scope_stack: Vec<NonNull<Scope>>,
}

impl SymbolResolver {
    fn new() -> SymbolResolver {
        let mut file_scope = Scope::new();
        file_scope.set_father(&BULITIN_SCOPE);
        let cur_scope = NonNull::from(&file_scope);
        SymbolResolver {
            file_scope,
            cur_scope,
            scope_stack: vec![],
        }
    }

    fn push_scope(&mut self, scope: &Scope) {
        self.scope_stack.push(self.cur_scope);
        self.cur_scope = NonNull::from(scope);
    }

    fn pop_scope(&mut self) {
        if let Some(s) = self.scope_stack.pop() {
            self.cur_scope = s;
        } else {
            debug_assert!(false, "scope_stack is empty!");
        }
    }
}

impl Visit for SymbolResolver {
    fn visit_file(&mut self, file: &mut File) {
        for item in file.items.iter_mut() {
            // add global type definition
            match item {
                Item::Fn(item_fn) => self.file_scope.add_type_fn(item_fn),
                Item::Struct(item_struct) => self.file_scope.add_type_struct(item_struct),
                _ => {}
            }
            self.visit_item(item);
        }
    }

    fn visit_item(&mut self, item: &mut Item) {
        match item {
            Item::Fn(item_fn) => self.visit_item_fn(item_fn),
            Item::Struct(item_struct) => self.visit_item_struct(item_struct),
            _ => unimplemented!(),
        }
    }

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) {
        self.add_type_fn(item_fn);
        if let Some(block) = item_fn.fn_block.as_mut() {
            self.visit_block_expr(block);
        }
    }

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        unimplemented!()
    }

    /// visit the expression which may contain block expression
    fn visit_expr(&mut self, expr: &mut Expr) {
        match expr {
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
            _ => {}
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) {
        self.visit_expr(&mut unary_expr.expr);
    }

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) {
        block_expr.scope.set_father_from_non_null(self.cur_scope);
        self.push_scope(&block_expr.scope);
        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt);
        }
        if let Some(expr) = block_expr.expr_without_block.as_mut() {
            self.visit_expr(expr);
        }
        self.pop_scope();
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) {
        self.visit_expr(&mut assign_expr.lhs);
        self.visit_expr(&mut assign_expr.rhs);
    }

    fn visit_range_expr(&mut self, range_expr: &mut RangeExpr) {
        if let Some(expr) = range_expr.lhs.as_mut() {
            self.visit_expr(expr);
        }
        if let Some(expr) = range_expr.rhs.as_mut() {
            self.visit_expr(expr);
        }
    }

    fn visit_bin_op_expr(&mut self, bin_op_expr: &mut BinOpExpr) {
        self.visit_expr(&mut bin_op_expr.lhs);
        self.visit_expr(&mut bin_op_expr.rhs);
    }

    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr) {
        self.visit_expr(grouped_expr)
    }

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr) {
        for e in array_expr.elems.iter_mut() {
            self.visit_expr(e);
        }
        if let Some(expr) = array_expr.len_expr.expr.as_mut() {
            self.visit_expr(expr);
        }
    }

    fn visit_array_index_expr(&mut self, array_index_expr: &mut ArrayIndexExpr) {
        unimplemented!()
    }

    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr) {
        unimplemented!()
    }

    fn visit_tuple_index_expr(&mut self, tuple_index_expr: &mut TupleIndexExpr) {
        unimplemented!()
    }

    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr) {
        unimplemented!()
    }

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) {
        unimplemented!()
    }

    fn visit_field_access_expr(&mut self, field_access_expr: &mut FieldAccessExpr) {
        unimplemented!()
    }

    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr) {
        unimplemented!()
    }

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) {
        unimplemented!()
    }

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) {
        unimplemented!()
    }

    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr) {
        if let Some(expr) = return_expr.0.as_mut() {
            self.visit_expr(expr);
        }
    }

    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr) {
        if let Some(expr) = break_expr.0.as_mut() {
            self.visit_expr(expr);
        }
    }
}

impl SymbolResolver {
    fn add_type_fn(&mut self, item_fn: &ItemFn) {
        unsafe { self.cur_scope.as_mut().add_type_fn(item_fn) }
    }
}
