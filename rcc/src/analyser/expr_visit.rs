use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::{AssignExpr, BlockExpr, Expr, ExprKind, LhsExpr, PathExpr, UnAryExpr};

pub trait ExprVisit {
    fn type_info(&self) -> TypeInfo;
    /// mutable place expr, immutable place expr or value expr
    fn kind(&self) -> ExprKind;
}

impl ExprVisit for Expr {
    fn type_info(&self) -> TypeInfo {
        match self {
            Self::Path(e) => e.type_info(),
            Self::LitStr(_) => TypeInfo::Str,
            Self::LitChar(_) => TypeInfo::Char,
            Self::LitBool(_) => TypeInfo::Bool,
            Self::LitNum(ln) => TypeInfo::LitNum(ln.ret_type),
            Self::Unary(e) => e.type_info(),
            Self::Block(e) => e.type_info(),
            Self::Assign(e) => e.type_info(),
            // Self::Range(e) => e.ret_type(),
            // Self::BinOp(e) => e.ret_type(),
            // Self::Grouped(e) => e.ret_type(),
            // Self::Array(e) => e.ret_type(),
            // Self::ArrayIndex(e) => e.ret_type(),
            // Self::Tuple(e) => e.ret_type(),
            // Self::TupleIndex(e) => e.ret_type(),
            // Self::Struct(e) => e.ret_type(),
            // Self::Call(e) => e.ret_type(),
            // Self::FieldAccess(e) => e.ret_type(),
            // Self::While(e) => e.ret_type(),
            // Self::Loop(e) => e.ret_type(),
            // Self::If(e) => e.ret_type(),
            // Self::Return(e) => e.ret_type(),
            // Self::Break(e) => e.ret_type(),
            _ => todo!(),
        }
    }

    fn kind(&self) -> ExprKind {
        match self {
            Self::Path(e) => e.kind(),
            Self::LitStr(_) | Self::LitChar(_) | Self::LitBool(_) | Self::LitNum(_) => ExprKind::Value,
            Self::Unary(u) => u.kind(),
            Self::Block(b) => b.kind(),
            Self::Assign(a) => a.kind(),
            _ => todo!(),
        }
    }
}

impl ExprVisit for LhsExpr {
    fn type_info(&self) -> TypeInfo {
        match self {
            LhsExpr::Path(p) => p.type_info(),
            _ => todo!(),
        }
    }

    fn kind(&self) -> ExprKind {
        match self {
            LhsExpr::Path(p) => p.kind(),
            _ => todo!(),
        }
    }
}

impl ExprVisit for PathExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        self.expr_kind
    }
}

impl ExprVisit for UnAryExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        todo!()
    }
}

impl ExprVisit for BlockExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

impl ExprVisit for AssignExpr {
    fn type_info(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Place
    }
}
