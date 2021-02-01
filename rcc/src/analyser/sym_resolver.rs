use crate::analyser::scope::Scope;
use crate::analyser::sym_resolver::TypeInfo::Unknown;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BlockExpr, BreakExpr, CallExpr, Expr,
    FieldAccessExpr, GroupedExpr, IfExpr, LitNumExpr, LoopExpr, PathExpr, RangeExpr, ReturnExpr,
    StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemFn, ItemStruct};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::{TypeAnnotation, TypeFnPtr, TypeLitNum};
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
    _type: TypeInfo,
}

impl VarInfo {
    pub fn new(stmt_id: u64, kind: VarKind, _type: TypeInfo) -> VarInfo {
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

#[derive(Debug, PartialEq, Clone)]
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
    /// !
    Never,
    /// ()
    Unit,
    Bool,
    Str,
    Char,
    LitNum(TypeLitNum),
    Unknown,
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

fn to_type_info(type_anno: &TypeAnnotation, cur_scope: &Scope) -> TypeInfo {
    match type_anno {
        TypeAnnotation::Identifier(s) => cur_scope.find_typedef(s),
        TypeAnnotation::Never => TypeInfo::Never,
        TypeAnnotation::Bool => TypeInfo::Bool,
        TypeAnnotation::Str => TypeInfo::Str,
        TypeAnnotation::Char => TypeInfo::Char,
        TypeAnnotation::LitNum(t) => TypeInfo::LitNum(*t),
        TypeAnnotation::Unknown => TypeInfo::Unknown,
        _ => todo!(),
    }
}

pub struct SymbolResolver<'ast> {
    cur_scope: *mut Scope,
    file_scope: Option<&'ast mut Scope>,
    scope_stack: Vec<*mut Scope>,
}

impl<'ast> SymbolResolver<'ast> {
    pub fn new() -> SymbolResolver<'ast> {
        SymbolResolver {
            cur_scope: std::ptr::null_mut(),
            file_scope: None,
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
            unsafe { &mut *self.cur_scope }.cur_stmt_id = 0;
        } else {
            debug_assert!(false, "scope_stack is empty!");
        }
    }

    fn cur_scope_is_global(&mut self) -> bool {
        if let Some(f) = &mut self.file_scope {
            self.cur_scope == *f
        } else {
            false
        }
    }
}

impl<'ast> SymbolResolver<'ast> {
    pub fn visit_file(&mut self, file: &'ast mut File) -> Result<(), RccError> {
        self.cur_scope = &mut file.scope;
        self.file_scope = Some(&mut file.scope);

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
            Stmt::ExprStmt(expr) => {
                self.visit_expr(expr)?;
                Ok(())
            }
        }
    }

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError> {
        let mut type_info = if let Some(expr) = &mut let_stmt.expr {
            self.visit_expr(expr)?
        } else {
            Unknown
        };
        // TODO: process type annotation
        // if let Some(type_anno) = &let_stmt._type {}

        match &let_stmt.pattern {
            Pattern::Identifier(ident_pattern) => unsafe {
                (*self.cur_scope).add_variable(
                    ident_pattern.ident(),
                    if ident_pattern.is_mut() {
                        VarKind::LocalMut
                    } else {
                        VarKind::Local
                    },
                    type_info,
                );
            },
        }
        Ok(())
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<TypeInfo, RccError> {
        match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::LitNum(lit_expr) => Ok(TypeInfo::LitNum(lit_expr.ret_type)),
            Expr::LitBool(_) => Ok(TypeInfo::Bool),
            Expr::LitChar(_) => Ok(TypeInfo::Char),
            Expr::LitStr(_) => Ok(TypeInfo::Str),
            // Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Block(block_expr) => self.visit_block_expr(block_expr),
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            // Expr::Range(range_expr) => self.visit_range_expr(range_expr),
            // Expr::BinOp(bin_op_expr) => self.visit_bin_op_expr(bin_op_expr),
            // Expr::Grouped(grouped_expr) => self.visit_grouped_expr(grouped_expr),
            // Expr::Array(array_expr) => self.visit_array_expr(array_expr),
            // Expr::ArrayIndex(array_index_expr) => self.visit_array_index_expr(array_index_expr),
            // Expr::Tuple(tuple_expr) => self.visit_tuple_expr(tuple_expr),
            // Expr::TupleIndex(tuple_index_expr) => self.visit_tuple_index_expr(tuple_index_expr),
            // Expr::Struct(struct_expr) => self.visit_struct_expr(struct_expr),
            // Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            // Expr::FieldAccess(field_access_expr) => self.visit_field_access_expr(field_access_expr),
            // Expr::While(while_expr) => self.visit_while_expr(while_expr),
            // Expr::Loop(loop_expr) => self.visit_loop_expr(loop_expr),
            // Expr::If(if_expr) => self.visit_if_expr(if_expr),
            // Expr::Return(return_expr) => self.visit_return_expr(return_expr),
            // Expr::Break(break_expr) => self.visit_break_expr(break_expr),
            _ => Ok(TypeInfo::Unknown),
        }
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<TypeInfo, RccError> {
        if let Some(ident) = path_expr.segments.last() {
            if let Some(var_info) = unsafe { (*self.cur_scope).find_variable(ident) } {
                Ok(var_info._type.clone())
            } else {
                Err(format!("identifier `{}` not found", ident).into())
            }
        } else {
            Err("invalid ident".into())
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<TypeInfo, RccError> {
        self.visit_expr(&mut unary_expr.expr)
    }

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) -> Result<TypeInfo, RccError> {
        self.enter_block(block_expr);
        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt)?;
            unsafe { &mut *self.cur_scope }.cur_stmt_id += 1;
        }
        if let Some(expr) = block_expr.expr_without_block.as_mut() {
            self.visit_expr(expr)?;
            unsafe { &mut *self.cur_scope }.cur_stmt_id += 1;
        }
        self.exit_block();
        Ok(Unknown)
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<TypeInfo, RccError> {
        self.visit_expr(&mut assign_expr.lhs)?;
        self.visit_expr(&mut assign_expr.rhs)?;
        Ok(TypeInfo::Unit)
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

    fn visit_bin_op_expr(&mut self, bin_op_expr: &mut BinOpExpr) -> Result<TypeInfo, RccError> {
        self.visit_expr(&mut bin_op_expr.lhs)?;
        self.visit_expr(&mut bin_op_expr.rhs)
    }

    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr) -> Result<TypeInfo, RccError> {
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
