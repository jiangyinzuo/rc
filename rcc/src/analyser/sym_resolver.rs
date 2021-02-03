use crate::analyser::expr_visit::ExprVisit;
use crate::analyser::scope::Scope;
use crate::analyser::sym_resolver::TypeInfo::Unknown;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, BinOpExpr, BinOperator, BlockExpr, BreakExpr, CallExpr,
    Expr, ExprKind, FieldAccessExpr, GroupedExpr, IfExpr, LhsExpr, LitNumExpr, LoopExpr, PathExpr,
    RangeExpr, ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, UnOp, WhileExpr,
};
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemFn, ItemStruct, TypeEnum};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::{PtrKind, TypeAnnotation, TypeFnPtr, TypeLitNum};
use crate::ast::visit::Visit;
use crate::ast::Visibility;
use crate::rcc::RccError;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TypeInfo {
    Fn {
        vis: Visibility,
        inner: TypeFnPtr,
    },

    FnPtr(TypeFnPtr),

    Struct {
        vis: Visibility,
        fields: NonNull<Fields>,
    },

    Enum(TypeEnum),

    Ptr {
        kind: PtrKind,
        type_info: Box<TypeInfo>,
    },

    /// primitive type
    /// !
    Never,
    Str,
    /// ()
    Unit,
    Bool,
    Char,
    LitNum(TypeLitNum),
    Unknown,
}

impl TypeInfo {
    pub(crate) fn from_type_anno(type_anno: &TypeAnnotation, cur_scope: &Scope) -> TypeInfo {
        match type_anno {
            TypeAnnotation::Identifier(s) => cur_scope.find_def_except_fn(s),
            TypeAnnotation::Never => TypeInfo::Never,
            TypeAnnotation::Unit => TypeInfo::Unit,
            TypeAnnotation::Bool => TypeInfo::Bool,
            TypeAnnotation::Str => TypeInfo::ref_str(),
            TypeAnnotation::Char => TypeInfo::Char,
            TypeAnnotation::Ptr(tp) => TypeInfo::Ptr {
                kind: tp.ptr_kind,
                type_info: Box::new(TypeInfo::from_type_anno(&tp.type_anno, cur_scope)),
            },
            TypeAnnotation::Unknown => TypeInfo::Unknown,
            _ => todo!(),
        }
    }

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

    pub fn ref_str() -> TypeInfo {
        TypeInfo::Ptr {
            kind: PtrKind::Ref,
            type_info: Box::new(TypeInfo::Str),
        }
    }

    pub fn is_integer(&self) -> bool {
        if let TypeInfo::LitNum(ln) = &self {
            matches!(
                ln,
                TypeLitNum::I
                    | TypeLitNum::I8
                    | TypeLitNum::I16
                    | TypeLitNum::I32
                    | TypeLitNum::I64
                    | TypeLitNum::I128
                    | TypeLitNum::Isize
                    | TypeLitNum::U8
                    | TypeLitNum::U16
                    | TypeLitNum::U32
                    | TypeLitNum::U64
                    | TypeLitNum::U128
                    | TypeLitNum::Usize
            )
        } else {
            false
        }
    }

    pub fn is_float(&self) -> bool {
        if let TypeInfo::LitNum(ln) = &self {
            matches!(ln, TypeLitNum::F | TypeLitNum::F32 | TypeLitNum::F64)
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(&self, Self::LitNum(_))
    }

    pub fn is_i(&self) -> bool {
        if let TypeInfo::LitNum(ln) = &self {
            ln == &TypeLitNum::I
        } else {
            false
        }
    }

    pub fn is_f(&self) -> bool {
        if let TypeInfo::LitNum(ln) = &self {
            ln == &TypeLitNum::F
        } else {
            false
        }
    }
}

/// Fill the `type information` and `expr kind` attributes of the expr nodes on AST
pub struct SymbolResolver<'ast> {
    cur_scope: *mut Scope,
    file_scope: Option<&'ast mut Scope>,
    scope_stack: Vec<*mut Scope>,

    /// TODO: Operator override tables
    pub override_bin_ops: HashSet<(BinOperator, TypeInfo, TypeInfo)>,
    pub str_constants: HashMap<String, u64>,
}

impl<'ast> SymbolResolver<'ast> {
    pub fn new() -> SymbolResolver<'ast> {
        SymbolResolver {
            cur_scope: std::ptr::null_mut(),
            file_scope: None,
            scope_stack: vec![],
            override_bin_ops: HashSet::new(),
            str_constants: HashMap::new(),
        }
    }

    /// return `TypeInfo::Unknown` if bin_op expr is invalid
    fn primitive_bin_ops(lhs: &mut Expr, bin_op: BinOperator, rhs: &mut Expr) -> TypeInfo {
        let l_type = lhs.type_info();
        let r_type = rhs.type_info();
        match bin_op {
            // 3i64 << 2i32
            BinOperator::Shl | BinOperator::Shr => {
                if l_type.is_integer() && r_type.is_integer() {
                    l_type
                } else {
                    Unknown
                }
            }
            BinOperator::Plus | BinOperator::Minus | BinOperator::Star | BinOperator::Slash => {
                if let TypeInfo::LitNum(l_lit) = l_type {
                    if let TypeInfo::LitNum(r_lit) = r_type {
                        return if l_lit == r_lit {
                            l_type
                        } else if l_lit == TypeLitNum::I && r_lit.is_integer()
                            || l_lit == TypeLitNum::F && r_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = lhs {
                                expr.ret_type = r_lit;
                            }
                            r_type
                        } else if r_lit == TypeLitNum::I && l_lit.is_integer()
                            || r_lit == TypeLitNum::F && l_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = rhs {
                                expr.ret_type = l_lit;
                            }
                            l_type
                        } else {
                            Unknown
                        };
                    }
                }
                Unknown
            }
            _ => todo!(),
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
        for param in item_fn.fn_params.params.iter() {
            match &param.pattern {
                Pattern::Identifier(ident_pattern) => item_fn.fn_block.scope.add_variable(
                    ident_pattern.ident(),
                    if ident_pattern.is_mut() {
                        VarKind::LocalMut
                    } else {
                        VarKind::Local
                    },
                    TypeInfo::from_type_anno(&param._type, unsafe { &*self.cur_scope }),
                ),
            }
        }
        self.visit_block_expr(&mut item_fn.fn_block)?;
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
            self.visit_expr(expr)?;
            expr.type_info()
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
                    type_info.clone(),
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

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<(), RccError> {
        let res = match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::LitNum(_) => Ok(()),
            Expr::LitBool(_) => Ok(()),
            Expr::LitChar(_) => Ok(()),
            Expr::LitStr(s) => {
                if !self.str_constants.contains_key(s) {
                    self.str_constants
                        .insert(s.clone(), self.str_constants.len() as u64);
                }
                Ok(())
            }
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Block(block_expr) => self.visit_block_expr(block_expr),
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr),
            // Expr::Range(range_expr) => self.visit_range_expr(range_expr),
            Expr::BinOp(bin_op_expr) => self.visit_bin_op_expr(bin_op_expr),
            // Expr::Grouped(grouped_expr) => self.visit_grouped_expr(grouped_expr),
            // Expr::Array(array_expr) => self.visit_array_expr(array_expr),
            // Expr::ArrayIndex(array_index_expr) => self.visit_array_index_expr(array_index_expr),
            // Expr::Tuple(tuple_expr) => self.visit_tuple_expr(tuple_expr),
            // Expr::TupleIndex(tuple_index_expr) => self.visit_tuple_index_expr(tuple_index_expr),
            // Expr::Struct(struct_expr) => self.visit_struct_expr(struct_expr),
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            // Expr::FieldAccess(field_access_expr) => self.visit_field_access_expr(field_access_expr),
            // Expr::While(while_expr) => self.visit_while_expr(while_expr),
            // Expr::Loop(loop_expr) => self.visit_loop_expr(loop_expr),
            // Expr::If(if_expr) => self.visit_if_expr(if_expr),
            // Expr::Return(return_expr) => self.visit_return_expr(return_expr),
            // Expr::Break(break_expr) => self.visit_break_expr(break_expr),
            _ => Ok(()),
        };
        debug_assert_ne!(
            ExprKind::Unknown,
            expr.kind(),
            "unknown expr kind: {:?}",
            expr
        );
        res
    }

    fn visit_lhs_expr(&mut self, lhs_expr: &mut LhsExpr) -> Result<(), RccError> {
        match lhs_expr {
            LhsExpr::Path(expr) => self.visit_path_expr(expr)?,
            _ => todo!(),
        }
        Ok(())
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<(), RccError> {
        if let Some(ident) = path_expr.segments.last() {
            let cur_scope = unsafe { &mut *self.cur_scope };
            if let Some(var_info) = cur_scope.find_variable(ident) {
                path_expr.type_info = var_info._type.clone();
                path_expr.expr_kind = match var_info.kind {
                    VarKind::Static | VarKind::LocalMut => ExprKind::MutablePlace,
                    VarKind::Const | VarKind::Local => ExprKind::Place,
                };
                Ok(())
            } else {
                let type_info = cur_scope.find_fn(ident);
                if type_info != Unknown {
                    path_expr.type_info = type_info;
                    path_expr.expr_kind = ExprKind::Value;
                    Ok(())
                } else {
                    Err(format!("identifier `{}` not found", ident).into())
                }
            }
        } else {
            Err("invalid ident".into())
        }
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<(), RccError> {
        self.visit_expr(&mut unary_expr.expr)?;
        let type_info = unary_expr.expr.type_info();
        match unary_expr.op {
            UnOp::Deref => {
                if let TypeInfo::Ptr { kind: _, type_info } = type_info {
                    unary_expr.type_info = *type_info;
                } else {
                    return Err(format!("type `{:?}` can not be dereferenced", type_info).into());
                }
            }
            UnOp::Not => match type_info {
                TypeInfo::Bool | TypeInfo::LitNum(_) => {
                    unary_expr.type_info = type_info.clone();
                }
                t => {
                    return Err(format!("cannot apply unary operator `!` to type `{:?}`", t).into())
                }
            },
            UnOp::Neg => match type_info {
                TypeInfo::LitNum(_) => unary_expr.type_info = type_info.clone(),
                t => {
                    return Err(format!("cannot apply unary operator `-` to type `{:?}`", t).into())
                }
            },
            UnOp::Borrow => {
                unary_expr.type_info = TypeInfo::Ptr {
                    kind: PtrKind::Ref,
                    type_info: Box::new(type_info.clone()),
                };
            }
            UnOp::BorrowMut => {
                // TODO
            }
        }
        Ok(())
    }

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) -> Result<(), RccError> {
        self.enter_block(block_expr);
        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt)?;
            unsafe { &mut *self.cur_scope }.cur_stmt_id += 1;
        }
        if let Some(expr) = block_expr.expr_without_block.as_mut() {
            self.visit_expr(expr)?;
            unsafe { &mut *self.cur_scope }.cur_stmt_id += 1;
            block_expr.type_info = expr.type_info();
        } else if block_expr.stmts.is_empty() {
            block_expr.type_info = TypeInfo::Unit;
        } else {
            block_expr.type_info = match block_expr.stmts.last().unwrap() {
                Stmt::Semi | Stmt::Let(_) | Stmt::Item(_) => TypeInfo::Unit,
                Stmt::ExprStmt(e) => e.type_info(),
            };
        }

        self.exit_block();
        Ok(())
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<(), RccError> {
        self.visit_lhs_expr(&mut assign_expr.lhs)?;
        match assign_expr.lhs.kind() {
            ExprKind::Place => Err(RccError("lhs is not mutable".into())),
            ExprKind::Value => Err(RccError("can not assign to lhs".into())),
            ExprKind::Unknown => unreachable!("lhs kind should not be unknown"),
            ExprKind::MutablePlace => {
                self.visit_expr(&mut assign_expr.rhs)?;
                let l_type = assign_expr.lhs.type_info();
                let r_type = assign_expr.rhs.type_info();

                debug_assert_ne!(TypeInfo::Unknown, r_type);

                // let mut a; a = 32;
                if l_type == TypeInfo::Unknown {
                    assign_expr.lhs.set_type_info(r_type);
                } else if l_type != r_type {
                    if l_type.is_integer() && r_type.is_integer() {
                        // let mut a = 32; a = 64i128;
                        if l_type.is_i() {
                            assign_expr.lhs.set_type_info(r_type);
                        } else { // let mut a: i64; a = 32;
                           assign_expr.rhs.set_type_info(l_type);
                        }
                    } else if l_type.is_float() && r_type.is_float() {
                        // let mut a = 32.3; a = 33f32;
                        if l_type.is_f() {
                            assign_expr.lhs.set_type_info(r_type);
                        } else { // let mut a: f32; a = 33.2;
                            assign_expr.rhs.set_type_info(l_type);
                        }
                    } else {
                        return Err("invalid type in assign expr".into());
                    }
                }
                Ok(())
            }
        }
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
        self.visit_expr(&mut bin_op_expr.rhs)?;

        bin_op_expr.type_info = Self::primitive_bin_ops(
            &mut bin_op_expr.lhs,
            bin_op_expr.bin_op,
            &mut bin_op_expr.rhs,
        );
        // primitive bin_op || override bin_op
        if bin_op_expr.type_info != Unknown
            || self.override_bin_ops.contains(&(
                bin_op_expr.bin_op,
                bin_op_expr.lhs.type_info(),
                bin_op_expr.rhs.type_info(),
            ))
        {
            Ok(())
        } else {
            Err(format!("invalid operand for `{:?}`", bin_op_expr.bin_op).into())
        }
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
        todo!()
    }

    fn visit_tuple_expr(&mut self, tuple_expr: &mut TupleExpr) -> Result<(), RccError> {
        todo!()
    }

    fn visit_tuple_index_expr(
        &mut self,
        tuple_index_expr: &mut TupleIndexExpr,
    ) -> Result<(), RccError> {
        todo!()
    }

    fn visit_struct_expr(&mut self, struct_expr: &mut StructExpr) -> Result<(), RccError> {
        todo!()
    }

    fn visit_call_expr(&mut self, call_expr: &mut CallExpr) -> Result<(), RccError> {
        self.visit_expr(&mut call_expr.expr)?;
        if !call_expr.expr.is_callable() {
            return Err("expr is not callable".into());
        }

        let type_fn_ptr = match call_expr.expr.type_info() {
            TypeInfo::FnPtr(fn_ptr) => fn_ptr,
            TypeInfo::Fn { vis: _, inner } => inner,
            _ => unreachable!("callable type can only be fn_ptr or fn"),
        };

        for (expr, param) in call_expr
            .call_params
            .iter_mut()
            .zip(type_fn_ptr.params.iter())
        {
            self.visit_expr(expr)?;
            let excepted_info = TypeInfo::from_type_anno(param, unsafe { &*self.cur_scope });

            fn check_and_change_type_info(
                expr_chg: &mut Expr,
                excepted_info: &TypeInfo,
            ) -> Result<(), RccError> {
                if excepted_info != &expr_chg.type_info() {
                    if let Expr::LitNum(lit_expr) = expr_chg {
                        if let TypeInfo::LitNum(p) = excepted_info {
                            if lit_expr.ret_type == TypeLitNum::I && p.is_integer()
                                || lit_expr.ret_type == TypeLitNum::F && p.is_integer()
                            {
                                lit_expr.ret_type = *p;
                                return Ok(());
                            }
                        }
                    }
                    return Err(format!(
                        "invalid type: execpted {:?}, found: {:?}",
                        excepted_info,
                        expr_chg.type_info()
                    )
                    .into());
                } else {
                    Ok(())
                }
            }

            check_and_change_type_info(expr, &excepted_info)?;
        }
        call_expr.type_info =
            TypeInfo::from_type_anno(&type_fn_ptr.ret_type, unsafe { &*self.cur_scope });
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
