use crate::analyser::scope::{Scope, ScopeStack};
use crate::analyser::sym_resolver::LoopKind::NotIn;
use crate::analyser::sym_resolver::TypeInfo::Unknown;
use crate::ast::expr::{
    ArrayExpr, ArrayIndexExpr, AssignExpr, AssignOp, BinOpExpr, BinOperator, BlockExpr, BreakExpr,
    CallExpr, Expr, ExprKind, FieldAccessExpr, GroupedExpr, IfExpr, LhsExpr, LoopExpr, PathExpr,
    RangeExpr, ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, UnAryExpr, UnOp, WhileExpr,
};
use crate::ast::expr::{ExprVisit, TypeInfoSetter};
use crate::ast::file::File;
use crate::ast::item::{Fields, Item, ItemExternalBlock, ItemFn, ItemStruct, TypeEnum, ExternalItemFn, FnSignature, ExternalItem};
use crate::ast::pattern::{IdentPattern, Pattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::ast::types::{PtrKind, TypeAnnotation, TypeFnPtr, TypeLitNum};
use crate::ast::Visibility;
use crate::rcc::RccError;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VarKind {
    Static,
    Const,
    LitConst,
    LocalMut,
    Local,
}

#[derive(Debug, PartialEq)]
pub struct VarInfo {
    stmt_id: u64,
    kind: VarKind,
    pub type_info: Rc<RefCell<TypeInfo>>,
}

impl VarInfo {
    pub fn new(stmt_id: u64, kind: VarKind, type_info: Rc<RefCell<TypeInfo>>) -> VarInfo {
        VarInfo {
            stmt_id,
            kind,
            type_info,
        }
    }

    pub fn stmt_id(&self) -> u64 {
        self.stmt_id
    }

    pub fn kind(&self) -> VarKind {
        self.kind
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    pub(crate) fn from_fn_signature(item: &impl FnSignature) -> Self {
        let tp_fn_ptr = TypeFnPtr::from_fn_signature(item);
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

    /// type `!` can be coerced into any other type.
    pub fn is(&self, other: &Self) -> bool {
        self == &Self::Never || self == other
    }

    pub fn eq_or_never(&self, other: &Self) -> bool {
        self == other || self == &Self::Never || other == &Self::Never
    }

    pub fn is_unknown(&self) -> bool {
        self == &TypeInfo::Unknown
    }

    pub fn is_never(&self) -> bool {
        self == &TypeInfo::Never
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LoopKind {
    NotIn,
    While,
    Loop(*mut LoopExpr),
}

impl LoopKind {
    fn is_in_loop(&self) -> bool {
        self != &NotIn
    }
}

/// Fill the `type information` and `expr kind` attributes of the expr nodes on AST
pub struct SymbolResolver {
    scope_stack: ScopeStack,

    loop_kind: LoopKind,
    loop_kind_stack: Vec<LoopKind>,

    cur_fn_ret_type: TypeInfo,
    cur_fn_ret_type_stack: Vec<TypeInfo>,

    // TODO: Operator override tables
    pub override_bin_ops: HashSet<(BinOperator, TypeInfo, TypeInfo)>,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            scope_stack: ScopeStack::new(),
            loop_kind: NotIn,
            loop_kind_stack: vec![],
            cur_fn_ret_type: TypeInfo::Unknown,
            cur_fn_ret_type_stack: vec![],
            override_bin_ops: HashSet::new(),
        }
    }

    /// return `TypeInfo::Unknown` if bin_op expr is invalid
    fn primitive_bin_ops(
        lhs: &mut Expr,
        bin_op: BinOperator,
        rhs: &mut Expr,
    ) -> Rc<RefCell<TypeInfo>> {
        let l_type: Rc<RefCell<TypeInfo>> = lhs.type_info();
        let r_type: Rc<RefCell<TypeInfo>> = rhs.type_info();
        match bin_op {
            // 3i64 << 2i32
            BinOperator::Shl | BinOperator::Shr => {
                if l_type.borrow().deref().is_integer() && r_type.borrow().deref().is_integer() {
                    l_type
                } else {
                    Rc::new(RefCell::new(Unknown))
                }
            }
            BinOperator::Plus | BinOperator::Minus | BinOperator::Star | BinOperator::Slash => {
                if let TypeInfo::LitNum(l_lit) = l_type.borrow().deref() {
                    if let TypeInfo::LitNum(r_lit) = r_type.borrow().deref() {
                        return if l_lit == r_lit {
                            l_type.clone()
                        } else if l_lit == &TypeLitNum::I && r_lit.is_integer()
                            || l_lit == &TypeLitNum::F && r_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = lhs {
                                expr.set_type_info_ref(r_type.clone());
                            }
                            r_type.clone()
                        } else if r_lit == &TypeLitNum::I && l_lit.is_integer()
                            || r_lit == &TypeLitNum::F && l_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = rhs {
                                expr.set_type_info_ref(l_type.clone());
                            }
                            l_type.clone()
                        } else {
                            Rc::new(RefCell::new(Unknown))
                        };
                    }
                }
                Rc::new(RefCell::new(Unknown))
            }
            BinOperator::Percent => match (l_type.borrow().deref(), r_type.borrow().deref()) {
                (TypeInfo::LitNum(l_lit), TypeInfo::LitNum(r_lit)) => {
                    if l_lit == &TypeLitNum::I && r_lit.is_integer() {
                        lhs.set_type_info_ref(r_type.clone());
                    } else if r_lit == &TypeLitNum::I && l_lit.is_integer()  {
                        rhs.set_type_info_ref(l_type.clone())
                    } else if l_lit != r_lit || !l_lit.is_integer() {
                        return Rc::new(RefCell::new(Unknown))
                    }
                    lhs.type_info()
                }
                _ => Rc::new(RefCell::new(Unknown)),
            },
            BinOperator::Lt
            | BinOperator::Gt
            | BinOperator::Le
            | BinOperator::Ge
            | BinOperator::EqEq
            | BinOperator::Ne => {
                if let TypeInfo::LitNum(l_lit) = l_type.borrow().deref() {
                    if let TypeInfo::LitNum(r_lit) = r_type.borrow().deref() {
                        return if l_lit == r_lit {
                            Rc::new(RefCell::new(TypeInfo::Bool))
                        } else if l_lit == &TypeLitNum::I && r_lit.is_integer()
                            || l_lit == &TypeLitNum::F && r_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = lhs {
                                expr.set_type_info_ref(r_type.clone());
                            }
                            Rc::new(RefCell::new(TypeInfo::Bool))
                        } else if r_lit == &TypeLitNum::I && l_lit.is_integer()
                            || r_lit == &TypeLitNum::F && l_lit.is_float()
                        {
                            if let Expr::LitNum(expr) = rhs {
                                expr.set_type_info_ref(l_type.clone());
                            }
                            Rc::new(RefCell::new(TypeInfo::Bool))
                        } else {
                            Rc::new(RefCell::new(Unknown))
                        };
                    }
                }
                Rc::new(RefCell::new(Unknown))
            }
            BinOperator::And | BinOperator::Or | BinOperator::Caret => {
                if let TypeInfo::LitNum(l_lit) = l_type.borrow().deref() {
                    if let TypeInfo::LitNum(r_lit) = r_type.borrow().deref() {
                        return if l_lit == r_lit {
                            l_type.clone()
                        } else if l_lit == &TypeLitNum::I && r_lit.is_integer() {
                            if let Expr::LitNum(expr) = lhs {
                                expr.set_type_info_ref(l_type.clone());
                            }
                            r_type.clone()
                        } else if r_lit == &TypeLitNum::I && l_lit.is_integer() {
                            if let Expr::LitNum(expr) = rhs {
                                expr.set_type_info_ref(l_type.clone());
                            }
                            l_type.clone()
                        } else {
                            Rc::new(RefCell::new(Unknown))
                        };
                    }
                } else if l_type.borrow().deref() == &TypeInfo::Bool
                    && r_type.borrow().deref() == &TypeInfo::Bool
                {
                    return Rc::new(RefCell::new(TypeInfo::Bool));
                }
                Rc::new(RefCell::new(Unknown))
            }
            BinOperator::AndAnd | BinOperator::OrOr => {
                // if loop {} && true {}
                if l_type.borrow().deref().is(&TypeInfo::Bool)
                    && r_type.borrow().deref().is(&TypeInfo::Bool)
                {
                    return Rc::new(RefCell::new(TypeInfo::Bool));
                }
                Rc::new(RefCell::new(Unknown))
            }
            BinOperator::As => {
                todo!()
            }
        }
    }

    fn exit_loop(&mut self) {
        self.loop_kind = self.loop_kind_stack.pop().expect("empty loop kind stack!");
    }

    fn try_determine_number_type(
        expected_num_type: &TypeInfo,
        expr: &mut (impl ExprVisit + TypeInfoSetter),
    ) {
        let type_info = expr.type_info();
        let type_info = type_info.borrow();

        if expected_num_type.is_integer() && type_info.is_i()
            || expected_num_type.is_float() && type_info.is_f()
        {
            std::mem::drop(type_info);
            expr.set_type_info(expected_num_type.clone());
        }
    }

    fn validate_ret_type(&self, type_info: &TypeInfo) -> Result<(), RccError> {
        if type_info.is(&self.cur_fn_ret_type) {
            Ok(())
        } else {
            Err(format!(
                "invalid return type: excepted `{:?}`, found `{:?}`",
                self.cur_fn_ret_type, type_info
            )
            .into())
        }
    }
}

impl SymbolResolver {
    pub(crate) fn visit_file(&mut self, file: &mut File) -> Result<(), RccError> {
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
            Item::ExternalBlock(external_block) => self.visit_item_external_block(external_block),
            _ => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<(), RccError> {
        let result = match expr {
            Expr::Path(path_expr) => self.visit_path_expr(path_expr),
            Expr::LitNum(lit_num_expr) => Ok(()),
            Expr::LitBool(lit_bool) => Ok(()),
            Expr::LitChar(lig_char) => Ok(()),
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

    fn visit_lhs_expr(&mut self, lhs_expr: &mut LhsExpr) -> Result<(), RccError> {
        let r = match lhs_expr {
            LhsExpr::Path(expr) => self.visit_path_expr(expr)?,
            _ => todo!("visit lhs expr"),
        };
        Ok(r)
    }

    fn visit_grouped_expr(&mut self, grouped_expr: &mut GroupedExpr) -> Result<(), RccError> {
        self.visit_expr(grouped_expr)
    }

    fn visit_item_fn(&mut self, item_fn: &mut ItemFn) -> Result<(), RccError> {
        // enter
        let mut temp_ret_type = Unknown;
        std::mem::swap(&mut self.cur_fn_ret_type, &mut temp_ret_type);
        self.cur_fn_ret_type_stack.push(temp_ret_type);
        self.cur_fn_ret_type =
            TypeInfo::from_type_anno(&item_fn.ret_type, self.scope_stack.cur_scope());

        // visit params of function
        for param in item_fn.fn_params.params.iter() {
            match &param.pattern {
                Pattern::Identifier(ident_pattern) => item_fn.fn_block.scope.add_variable(
                    ident_pattern.ident(),
                    if ident_pattern.is_mut() {
                        VarKind::LocalMut
                    } else {
                        VarKind::Local
                    },
                    Rc::new(RefCell::new(TypeInfo::from_type_anno(
                        &param._type,
                        self.scope_stack.cur_scope(),
                    ))),
                ),
            }
        }
        self.visit_block_expr(&mut item_fn.fn_block)?;
        if item_fn.fn_block.last_expr.is_some() {
            Self::try_determine_number_type(&self.cur_fn_ret_type, &mut item_fn.fn_block);
            let type_info = item_fn.fn_block.type_info();
            let t = type_info.borrow();
            let tp = t.deref();
            self.validate_ret_type(tp)?;
        } else if item_fn.fn_block.stmts.is_empty() {
            if item_fn.ret_type != TypeAnnotation::Unit {
                return Err(format!(
                    "invalid return type: expected `{:?}`, found `()`",
                    item_fn.ret_type
                )
                .into());
            }
        } else {
            let last_stmt = item_fn.fn_block.stmts.last().unwrap();
            let type_info = last_stmt.type_info();
            self.validate_ret_type(&type_info)?;
        }

        // restore
        self.cur_fn_ret_type = self
            .cur_fn_ret_type_stack
            .pop()
            .expect("empty cur_fn_ret_type_stack!");
        Ok(())
    }

    fn visit_item_struct(&mut self, item_struct: &mut ItemStruct) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_item_external_block(
        &mut self,
        external_block: &mut ItemExternalBlock,
    ) -> Result<(), RccError> {
        for item in &mut external_block.external_items {
            match item {
                ExternalItem::Fn(f) => {
                    self.visit_external_item_fn(f)?;
                }
            }
        }
        Ok(())
    }

    fn visit_external_item_fn(&mut self, _external_item_fn: &mut ExternalItemFn) -> Result<(), RccError> {
        // do nothing
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RccError> {
        match stmt {
            Stmt::Semi => Ok(()),
            Stmt::Item(item) => self.visit_item(item),
            Stmt::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Stmt::ExprStmt(expr) => {
                self.visit_expr(expr)?;
                let t = expr.type_info();
                let tp = t.borrow();
                let type_info = tp.deref();
                if expr.with_block() && type_info != &TypeInfo::Unit && !type_info.is_never() {
                    return Err(format!(
                        "invalid type for expr stmt: expected `()`, found {:?}",
                        type_info
                    )
                    .into());
                }
                Ok(())
            }
        }
    }

    fn visit_let_stmt(&mut self, let_stmt: &mut LetStmt) -> Result<(), RccError> {
        let expr_type_info = if let Some(expr) = &mut let_stmt.rhs {
            self.visit_expr(expr)?;
            if let Some(type_anno) = &let_stmt._type {
                let anno_type_info =
                    TypeInfo::from_type_anno(type_anno, self.scope_stack.cur_scope());
                Self::try_determine_number_type(&anno_type_info, expr);
                let t = expr.type_info();
                let tp = t.borrow();
                let expr_type_info = tp.deref();
                if !expr_type_info.is(&anno_type_info) {
                    return Err(format!(
                        "invalid type in let stmt: expected `{:?}`, found `{:?}`",
                        anno_type_info, expr_type_info
                    )
                    .into());
                }
            }
            expr.type_info()
        } else {
            Rc::new(RefCell::new(Unknown))
        };

        match &let_stmt.pattern {
            Pattern::Identifier(ident_pattern) => {
                self.scope_stack.cur_scope_mut().add_variable(
                    ident_pattern.ident(),
                    if ident_pattern.is_mut() {
                        VarKind::LocalMut
                    } else {
                        VarKind::Local
                    },
                    expr_type_info,
                );
            }
        }
        Ok(())
    }

    fn visit_pattern(&mut self, pattern: &mut Pattern) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_ident_pattern(&mut self, ident_pattern: &mut IdentPattern) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_path_expr(&mut self, path_expr: &mut PathExpr) -> Result<(), RccError> {
        if let Some(ident) = path_expr.segments.last() {
            let cur_scope = self.scope_stack.cur_scope_mut();
            if let Some((var_info, _scope_id)) = cur_scope.find_variable(ident) {
                path_expr.set_type_info_ref(var_info.type_info.clone());
                path_expr.expr_kind = match var_info.kind {
                    VarKind::Static | VarKind::LocalMut => ExprKind::MutablePlace,
                    VarKind::Const | VarKind::Local => ExprKind::Place,
                    VarKind::LitConst => unreachable!(),
                };
                Ok(())
            } else {
                let type_info = cur_scope.find_fn(ident);
                if !type_info.is_unknown() {
                    path_expr.set_type_info(type_info);
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

    fn visit_lit_str(&mut self, _: &str) -> Result<(), RccError> {
        // do nothing
        Ok(())
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnAryExpr) -> Result<(), RccError> {
        self.visit_expr(&mut unary_expr.expr)?;
        let type_info = unary_expr.expr.type_info();
        match unary_expr.op {
            UnOp::Deref => {
                if let TypeInfo::Ptr { kind: _, type_info } = type_info.borrow().deref() {
                    unary_expr.set_type_info(*type_info.clone());
                    unary_expr.expr_kind = unary_expr.expr.kind();
                } else {
                    return Err(format!("type `{:?}` can not be dereferenced", type_info).into());
                }
            }
            UnOp::Not => match type_info.borrow().deref() {
                TypeInfo::Bool | TypeInfo::LitNum(_) => {
                    unary_expr.set_type_info_ref(type_info.clone());
                    unary_expr.expr_kind = ExprKind::Value;
                }
                t => {
                    return Err(format!("cannot apply unary operator `!` to type `{:?}`", t).into())
                }
            },
            UnOp::Neg => match type_info.borrow().deref() {
                TypeInfo::LitNum(_) => {
                    unary_expr.set_type_info_ref(type_info.clone());
                    unary_expr.expr_kind = ExprKind::Value;
                }
                tp => {
                    return Err(
                        format!("cannot apply unary operator `-` to type `{:?}`", tp).into(),
                    )
                }
            },
            UnOp::Borrow => {
                unary_expr.set_type_info(TypeInfo::Ptr {
                    kind: PtrKind::Ref,
                    type_info: Box::new(type_info.borrow().deref().clone()),
                });
                unary_expr.expr_kind = ExprKind::Value;
            }
            UnOp::BorrowMut => {
                todo!("borrow mut")
            }
        }
        Ok(())
    }

    fn visit_block_expr(&mut self, block_expr: &mut BlockExpr) -> Result<(), RccError> {
        self.scope_stack.enter_scope(block_expr);

        for stmt in block_expr.stmts.iter_mut() {
            self.visit_stmt(stmt)?;
            self.scope_stack.cur_scope_mut().cur_stmt_id += 1;
        }

        if let Some(expr) = block_expr.last_expr.as_mut() {
            self.visit_expr(expr)?;
            self.scope_stack.cur_scope_mut().cur_stmt_id += 1;
            let type_info = expr.type_info();
            block_expr.set_type_info_ref(type_info);
        } else if block_expr.stmts.is_empty() {
            block_expr.set_type_info(TypeInfo::Unit);
        } else {
            let last_stmt = block_expr.stmts.last().unwrap();
            match last_stmt {
                Stmt::Semi | Stmt::Let(_) | Stmt::Item(_) => {
                    block_expr.set_type_info(TypeInfo::Unit);
                }
                Stmt::ExprStmt(e) => block_expr.set_type_info_ref(e.type_info()),
            }
        }

        self.scope_stack.exit_scope();
        Ok(())
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr) -> Result<(), RccError> {
        fn invalid_type_error(
            type_info: &TypeInfo,
            assign_expr: &AssignExpr,
        ) -> Result<(), RccError> {
            Err(format!(
                "invalid type `{:?}` for `{:?}`",
                type_info, assign_expr.assign_op
            )
            .into())
        }

        self.visit_lhs_expr(&mut assign_expr.lhs)?;

        // check the mutability of place expr lhs
        match assign_expr.lhs.kind() {
            ExprKind::Place => return Err("lhs is not mutable".into()),
            ExprKind::Value => return Err("can not assign to lhs".into()),
            ExprKind::Unknown => unreachable!("lhs kind should not be unknown"),
            ExprKind::MutablePlace => {
                self.visit_expr(&mut assign_expr.rhs)?;
                let l_type = assign_expr.lhs.type_info();
                let r_type = assign_expr.rhs.type_info();

                debug_assert!(!r_type.borrow().deref().is_unknown());

                if matches!(assign_expr.assign_op, AssignOp::ShlEq | AssignOp::ShrEq) {
                    return if l_type.borrow().deref().is_integer()
                        && r_type.borrow().deref().is_integer()
                    {
                        Ok(())
                    } else {
                        invalid_type_error(l_type.borrow().deref(), assign_expr)
                    };
                }

                // set type_info of lhs or rhs

                // let mut a; a = 32;
                if l_type.borrow().deref().is_unknown() {
                    assign_expr.lhs.set_type_info_ref(r_type);
                } else if !r_type.borrow().deref().is(l_type.borrow().deref()) {
                    if l_type.borrow().deref().is_integer() && r_type.borrow().deref().is_integer()
                    {
                        // let mut a = 32; a = 64i128;
                        if l_type.borrow().deref().is_i() {
                            assign_expr.lhs.set_type_info_ref(r_type);
                        } else {
                            // let mut a: i64; a = 32;
                            assign_expr.rhs.set_type_info_ref(l_type);
                        }
                    } else if l_type.borrow().deref().is_float()
                        && r_type.borrow().deref().is_float()
                    {
                        // let mut a = 32.3; a = 33f32;
                        if l_type.borrow().deref().is_f() {
                            assign_expr.lhs.set_type_info_ref(r_type);
                        } else {
                            // let mut a: f32; a = 33.2;
                            assign_expr.rhs.set_type_info_ref(l_type);
                        }
                    } else {
                        return invalid_type_error(l_type.borrow().deref(), assign_expr);
                    }
                }
            }
        }

        debug_assert_eq!(assign_expr.lhs.type_info(), assign_expr.rhs.type_info());

        let t = assign_expr.lhs.type_info();
        let tp = t.borrow();
        let type_info = tp.deref();
        // check compound assignment operators
        // TODO: operator override
        match assign_expr.assign_op {
            AssignOp::PlusEq | AssignOp::MinusEq | AssignOp::StarEq | AssignOp::SlashEq => {
                if !type_info.is_number() {
                    return invalid_type_error(type_info, assign_expr);
                }
            }
            AssignOp::PercentEq | AssignOp::AndEq | AssignOp::OrEq | AssignOp::CaretEq => {
                if !type_info.is_integer() && type_info != &TypeInfo::Bool {
                    return invalid_type_error(type_info, assign_expr);
                }
            }
            AssignOp::ShlEq | AssignOp::ShrEq => unreachable!(),
            AssignOp::Eq => {}
        }
        Ok(())
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

        let t = Self::primitive_bin_ops(
            &mut bin_op_expr.lhs,
            bin_op_expr.bin_op,
            &mut bin_op_expr.rhs,
        );
        bin_op_expr.set_type_info_ref(t.clone());
        // primitive bin_op || override bin_op
        let tp = t.borrow();
        let bin_type = tp.deref();
        if !bin_type.is_unknown()
            || self.override_bin_ops.contains(&(
                bin_op_expr.bin_op,
                bin_op_expr.lhs.type_info().borrow().deref().clone(),
                bin_op_expr.rhs.type_info().borrow().deref().clone(),
            ))
        {
            Ok(())
        } else {
            Err(format!(
                "invalid operand type `{:?}` and `{:?}` for `{:?}`",
                bin_op_expr.lhs.type_info().borrow().deref(),
                bin_op_expr.rhs.type_info().borrow().deref(),
                bin_op_expr.bin_op
            )
            .into())
        }
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
        let t = call_expr.expr.type_info();
        let tp = t.borrow();
        let type_info = tp.deref();
        let type_fn_ptr = match type_info {
            TypeInfo::FnPtr(fn_ptr) => fn_ptr,
            TypeInfo::Fn { vis: _, inner } => inner,
            _ => unreachable!("callable type can only be fn_ptr or fn"),
        };

        if call_expr.call_params.len() != type_fn_ptr.params.len() {
            return Err(format!(
                "This function takes {} parameters but {} parameters was supplied",
                type_fn_ptr.params.len(),
                call_expr.call_params.len(),
            )
            .into());
        }
        for (expr, param) in call_expr
            .call_params
            .iter_mut()
            .zip(type_fn_ptr.params.iter())
        {
            self.visit_expr(expr)?;
            let excepted_info = TypeInfo::from_type_anno(param, self.scope_stack.cur_scope());

            Self::try_determine_number_type(&excepted_info, expr);
            assert_type_is(expr, &excepted_info, "invalid type for call expr")?;
        }
        call_expr.set_type_info(TypeInfo::from_type_anno(
            &type_fn_ptr.ret_type,
            self.scope_stack.cur_scope(),
        ));
        Ok(())
    }

    fn visit_field_access_expr(
        &mut self,
        field_access_expr: &mut FieldAccessExpr,
    ) -> Result<(), RccError> {
        Ok(())
    }

    fn visit_while_expr(&mut self, while_expr: &mut WhileExpr) -> Result<(), RccError> {
        self.visit_expr(&mut while_expr.0)?;
        // store loop kind
        self.loop_kind_stack.push(self.loop_kind);
        self.loop_kind = LoopKind::While;
        assert_type_is(
            &*while_expr.0,
            &TypeInfo::Bool,
            "invalid type in while condition",
        )?;

        self.visit_block_expr(&mut while_expr.1)?;
        assert_type_is(
            &*while_expr.1,
            &TypeInfo::Unit,
            "invalid type in while block",
        )?;

        // restore loop kind
        self.exit_loop();
        Ok(())
    }

    fn visit_loop_expr(&mut self, loop_expr: &mut LoopExpr) -> Result<(), RccError> {
        self.loop_kind_stack.push(self.loop_kind);
        self.loop_kind = LoopKind::Loop(loop_expr);
        self.visit_block_expr(&mut loop_expr.expr)?;
        // never return, example: `let a = loop {};`
        let t = loop_expr.type_info();
        let tp = t.borrow();
        let type_info = tp.deref();
        if type_info.is_unknown() {
            loop_expr.set_type_info(TypeInfo::Never);
        }
        self.exit_loop();
        Ok(())
    }

    fn visit_if_expr(&mut self, if_expr: &mut IfExpr) -> Result<(), RccError> {
        debug_assert!(
            if_expr.conditions.len() == if_expr.blocks.len()
                || if_expr.conditions.len() + 1 == if_expr.blocks.len(),
            "len cond: {}; len block: {}",
            if_expr.conditions.len(),
            if_expr.blocks.len()
        );
        debug_assert!(!if_expr.conditions.is_empty());
        debug_assert!(!if_expr.blocks.is_empty());

        for cond in if_expr.conditions.iter_mut() {
            self.visit_expr(cond)?;
            let t = cond.type_info();
            let tp = t.borrow();
            let cond_type_info = tp.deref();
            if !cond_type_info.is(&TypeInfo::Bool) {
                return Err(format!(
                    "invalid type of condition expr: expected `bool`, found: {:?}",
                    cond_type_info
                )
                .into());
            }
        }

        let mut block_type = TypeInfo::Unknown;
        for block in if_expr.blocks.iter_mut() {
            self.visit_block_expr(block)?;
            let type_info = block.type_info();
            let t = type_info.borrow();
            let tp = t.deref();
            debug_assert_ne!(&TypeInfo::Unknown, type_info.borrow().deref());

            if block_type != TypeInfo::Unknown && !block_type.eq_or_never(tp) {
                return Err(format!(
                    "different type of if block: `{:?}`, `{:?}`",
                    block_type, type_info
                )
                .into());
            }

            if tp != &TypeInfo::Never {
                block_type = tp.clone();
            }
        }

        if_expr.set_type_info(if block_type == TypeInfo::Unknown {
            TypeInfo::Never
        } else {
            block_type
        });
        Ok(())
    }

    fn visit_return_expr(&mut self, return_expr: &mut ReturnExpr) -> Result<(), RccError> {
        match return_expr.0.as_mut() {
            Some(expr) => {
                self.visit_expr(expr)?;
                Self::try_determine_number_type(&self.cur_fn_ret_type, expr.as_mut());
                let type_info = expr.type_info();
                let t = type_info.borrow();
                let tp = t.deref();
                self.validate_ret_type(tp)
            }
            None => self.validate_ret_type(&TypeInfo::Unit),
        }
    }

    fn visit_break_expr(&mut self, break_expr: &mut BreakExpr) -> Result<(), RccError> {
        fn try_set_type_info(
            loop_expr: *mut LoopExpr,
            type_info: Rc<RefCell<TypeInfo>>,
        ) -> Result<(), RccError> {
            let tp = type_info.borrow();
            let t = tp.deref();
            let loop_expr = unsafe { &mut *loop_expr };
            let l = loop_expr.type_info();
            let lt = l.borrow();
            let loop_type_info = lt.deref();
            if loop_type_info.is_unknown() {
                loop_expr.set_type_info_ref(type_info.clone());
                Ok(())
            } else if !t.is(loop_type_info) {
                Err(format!(
                    "invalid type for break expr: expected `{:?}`, found {:?}",
                    loop_type_info, t
                )
                .into())
            } else {
                Ok(())
            }
        }

        if !self.loop_kind.is_in_loop() {
            return Err("break expr can not be out of loop block".into());
        }

        if let Some(expr) = break_expr.0.as_mut() {
            return match self.loop_kind {
                LoopKind::Loop(loop_expr) => {
                    self.visit_expr(expr)?;
                    Self::try_determine_number_type(
                        unsafe { (*loop_expr).type_info().borrow().deref() },
                        expr.as_mut(),
                    );
                    try_set_type_info(loop_expr, expr.type_info())
                }
                _ => Err("only loop can return values".into()),
            };
        } else if let LoopKind::Loop(loop_expr) = self.loop_kind {
            return try_set_type_info(loop_expr, Rc::new(RefCell::new(TypeInfo::Unit)));
        }
        Ok(())
    }
}

pub(super) fn assert_type_is<T: ExprVisit>(
    expr: &T,
    expected_type: &TypeInfo,
    err_msg: &str,
) -> Result<(), RccError> {
    let type_info = expr.type_info();
    let t = type_info.borrow();
    let cond_type = t.deref();
    if !cond_type.is(expected_type) {
        return Err(format!(
            "{}: expected {:?}, found {:?}",
            err_msg, expected_type, cond_type
        )
        .into());
    }
    Ok(())
}
