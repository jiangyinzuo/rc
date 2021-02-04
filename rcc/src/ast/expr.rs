use crate::analyser::scope::Scope;
use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::Expr::Path;
use crate::ast::item::Item::Type;
use crate::ast::stmt::Stmt;
use crate::ast::types::TypeLitNum;
use crate::ast::{FromToken, TokenStart};
use crate::from_token;
use crate::lexer::token::Token;
use crate::rcc::RccError;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use strenum::StrEnum;

pub trait ExprVisit {
    fn type_info(&self) -> TypeInfo;
    /// mutable place expr, immutable place expr or value expr
    fn kind(&self) -> ExprKind;

    fn is_callable(&self) -> bool {
        matches!(self.type_info(), TypeInfo::Fn {..} | TypeInfo::FnPtr(_))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExprKind {
    MutablePlace,
    Place,
    Value,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Path(PathExpr),
    LitNum(LitNumExpr),
    LitBool(bool),
    LitChar(char),
    LitStr(String),
    Unary(UnAryExpr),
    Block(BlockExpr),
    Assign(AssignExpr),
    Range(RangeExpr),
    BinOp(BinOpExpr),
    Grouped(GroupedExpr),
    Array(ArrayExpr),
    ArrayIndex(ArrayIndexExpr),
    Tuple(TupleExpr),
    TupleIndex(TupleIndexExpr),
    Struct(StructExpr),
    EnumVariant,
    Call(CallExpr),
    MethodCall,
    FieldAccess(FieldAccessExpr),
    While(WhileExpr),
    Loop(LoopExpr),
    For,
    If(IfExpr),
    Match,
    Return(ReturnExpr),
    Break(BreakExpr),
}

impl Expr {
    pub fn with_block(&self) -> bool {
        matches!(self,
            Self::Block(_) | Self::Struct(_) | Self::While(_) |
            Self::Loop(_)  | Self::If(_) | Self::Match | Self::For)
    }

    pub fn set_type_info(&mut self, type_info: TypeInfo) {
        match self {
            Self::Path(p) => p.type_info = type_info,
            e => unimplemented!("set type_info on {:?}", e),
        }
    }
}

impl From<&str> for Expr {
    fn from(ident: &str) -> Self {
        Path(ident.into())
    }
}

impl ExprVisit for Expr {
    fn type_info(&self) -> TypeInfo {
        match self {
            Self::Path(e) => e.type_info(),
            Self::LitStr(_) => TypeInfo::ref_str(),
            Self::LitChar(_) => TypeInfo::Char,
            Self::LitBool(_) => TypeInfo::Bool,
            Self::LitNum(ln) => TypeInfo::LitNum(ln.ret_type),
            Self::Unary(e) => e.type_info(),
            Self::Block(e) => e.type_info(),
            Self::Assign(e) => e.type_info(),
            // Self::Range(e) => e.ret_type(),
            Self::BinOp(e) => e.type_info(),
            // Self::Grouped(e) => e.ret_type(),
            // Self::Array(e) => e.ret_type(),
            // Self::ArrayIndex(e) => e.ret_type(),
            // Self::Tuple(e) => e.ret_type(),
            // Self::TupleIndex(e) => e.ret_type(),
            // Self::Struct(e) => e.ret_type(),
            Self::Call(e) => e.type_info(),
            // Self::FieldAccess(e) => e.ret_type(),
            Self::While(e) => e.type_info(),
            Self::Loop(e) => e.type_info(),
            // Self::If(e) => e.ret_type(),
            Self::Return(e) => e.type_info(),
            Self::Break(e) => e.type_info(),
            _ => unimplemented!("{:?}", self),
        }
    }

    fn kind(&self) -> ExprKind {
        match self {
            Self::Path(e) => e.kind(),
            Self::LitStr(_) | Self::LitChar(_) | Self::LitBool(_) | Self::LitNum(_) => {
                ExprKind::Value
            }
            Self::Unary(u) => u.kind(),
            Self::Block(b) => b.kind(),
            Self::Assign(a) => a.kind(),
            Self::BinOp(b) => b.kind(),
            Self::Call(c) => c.kind(),
            Self::While(w) => w.kind(),
            Self::Loop(l) => l.kind(),
            Self::Return(r) => r.kind(),
            Self::Break(b) => b.kind(),
            _ => unimplemented!("{:?}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LhsExpr {
    Path(PathExpr),
    ArrayIndex(ArrayIndexExpr),
    TupleIndex(TupleIndexExpr),
    FieldAccess(FieldAccessExpr),
    Deref(Box<Expr>),
}

impl LhsExpr {
    pub fn from_expr(expr: Expr) -> Result<LhsExpr, RccError> {
        match expr {
            Expr::Path(p) => Ok(LhsExpr::Path(p)),
            Expr::Unary(u) => {
                if u.op == UnOp::Deref {
                    Ok(LhsExpr::Deref(u.expr))
                } else {
                    Err("invalid lhs expr".into())
                }
            }
            Expr::Grouped(e) => LhsExpr::from_expr(*e),
            Expr::ArrayIndex(e) => Ok(LhsExpr::ArrayIndex(e)),
            Expr::TupleIndex(e) => Ok(LhsExpr::TupleIndex(e)),
            Expr::FieldAccess(e) => Ok(LhsExpr::FieldAccess(e)),
            _ => Err("invalid lhs expr".into()),
        }
    }

    pub fn set_type_info(&mut self, type_info: TypeInfo) {
        match self {
            Self::Path(p) => p.type_info = type_info,
            Self::ArrayIndex(a) => unimplemented!("set array index type info"),
            Self::TupleIndex(t) => unimplemented!("set tuple index type info"),
            Self::FieldAccess(f) => unimplemented!("set tuple field type info"),
            Self::Deref(e) => unimplemented!("set tuple deref type info"),
        }
    }
}

impl ExprVisit for LhsExpr {
    fn type_info(&self) -> TypeInfo {
        match self {
            LhsExpr::Path(expr) => expr.type_info(),
            _ => todo!(),
        }
    }

    fn kind(&self) -> ExprKind {
        match self {
            LhsExpr::Path(expr) => expr.kind(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantExpr<V> {
    pub expr: Option<Box<Expr>>,
    const_value: Option<V>,
}

impl<V> ConstantExpr<V> {
    pub fn const_value(value: V) -> ConstantExpr<V> {
        ConstantExpr {
            expr: None,
            const_value: Some(value),
        }
    }

    pub fn expr(expr: Expr) -> ConstantExpr<V> {
        ConstantExpr {
            expr: Some(Box::new(expr)),
            const_value: None,
        }
    }
}

impl TokenStart for Expr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk,
            Token::Identifier(_) | Token::Literal {..} | Token::True | Token::False | Token::DotDot |
            Token::LeftCurlyBraces | Token::LeftParen | Token::LeftSquareBrackets |
            Token::For | Token::Loop | Token::While |
            Token::If | Token::Match | Token::Break | Token::Return
        ) || UnAryExpr::is_token_start(tk)
            || RangeExpr::is_token_start(tk)
    }
}

pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
    pub expr_without_block: Option<Box<Expr>>,
    pub scope: Scope,
    pub type_info: TypeInfo,
}

impl ExprVisit for BlockExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

impl Debug for BlockExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.expr_without_block {
            Some(expr) => write!(f, "{{ {:?} {:?} }}", self.stmts, expr),
            None => write!(f, "{{ {:?} }}", self.stmts),
        }
    }
}

impl PartialEq for BlockExpr {
    fn eq(&self, other: &Self) -> bool {
        self.stmts.eq(&other.stmts) && self.expr_without_block.eq(&other.expr_without_block)
    }
}

impl BlockExpr {
    pub fn new() -> BlockExpr {
        BlockExpr {
            stmts: vec![],
            expr_without_block: None,
            scope: Scope::new(),
            type_info: TypeInfo::Unknown,
        }
    }

    pub fn expr_without_block(mut self, expr: Expr) -> Self {
        debug_assert!(!expr.with_block());
        self.expr_without_block = Some(Box::new(expr));
        self
    }
}

impl From<Vec<Stmt>> for BlockExpr {
    fn from(stmts: Vec<Stmt>) -> Self {
        BlockExpr {
            stmts,
            expr_without_block: None,
            scope: Scope::new(),
            type_info: TypeInfo::Unknown,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LitNumExpr {
    pub ret_type: TypeLitNum,
    pub value: String,
}

impl LitNumExpr {
    pub fn new(value: String, ret_type: TypeLitNum) -> LitNumExpr {
        LitNumExpr { value, ret_type }
    }

    pub fn integer(value: String) -> LitNumExpr {
        LitNumExpr {
            ret_type: TypeLitNum::I,
            value,
        }
    }

    pub fn ret_type(mut self, ret_type: TypeLitNum) -> LitNumExpr {
        self.ret_type = ret_type;
        self
    }
}

impl From<i32> for LitNumExpr {
    fn from(num: i32) -> Self {
        LitNumExpr {
            ret_type: TypeLitNum::I,
            value: num.to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PathExpr {
    pub segments: Vec<String>,
    pub type_info: TypeInfo,
    pub expr_kind: ExprKind,
}

impl PathExpr {
    pub fn new() -> Self {
        PathExpr {
            segments: vec![],
            type_info: TypeInfo::Unknown,
            expr_kind: ExprKind::Unknown,
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

impl From<Vec<String>> for PathExpr {
    fn from(segments: Vec<String>) -> Self {
        PathExpr {
            segments,
            type_info: TypeInfo::Unknown,
            expr_kind: ExprKind::Unknown,
        }
    }
}

impl From<Vec<&str>> for PathExpr {
    fn from(segments: Vec<&str>) -> Self {
        PathExpr {
            segments: segments.iter().map(|s| s.to_string()).collect(),
            type_info: TypeInfo::Unknown,
            expr_kind: ExprKind::Unknown,
        }
    }
}

impl From<&str> for PathExpr {
    fn from(s: &str) -> Self {
        PathExpr {
            segments: s.split("::").map(|s| s.to_string()).collect(),
            type_info: TypeInfo::Unknown,
            expr_kind: ExprKind::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnAryExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
    pub type_info: TypeInfo,
    pub expr_kind: ExprKind,
}

impl UnAryExpr {
    pub fn new(op: UnOp, expr: Expr) -> Self {
        UnAryExpr {
            op,
            expr: Box::new(expr),
            type_info: TypeInfo::Unknown,
            expr_kind: ExprKind::Unknown,
        }
    }
}

impl ExprVisit for UnAryExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        self.expr_kind
    }
}

impl TokenStart for UnAryExpr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(
            tk,
            Token::Not | Token::Star | Token::Minus | Token::And | Token::AndAnd
        )
    }
}

#[derive(PartialEq)]
pub enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
    /// `&`
    Borrow,
    /// `& mut`
    BorrowMut,
}

impl Debug for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deref => "*",
                Self::Not => "!",
                Self::Neg => "-",
                Self::Borrow => "&",
                Self::BorrowMut => "& mut",
            }
        )
    }
}

impl FromToken for UnOp {
    fn from_token(tk: Token) -> Option<Self> {
        match tk {
            Token::Minus => Some(Self::Neg),
            Token::Star => Some(Self::Deref),
            Token::Not => Some(Self::Not),
            Token::And => Some(Self::Borrow),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignExpr {
    pub lhs: LhsExpr,
    pub assign_op: AssignOp,
    pub rhs: Box<Expr>,
}

impl AssignExpr {
    pub fn new(lhs: LhsExpr, assign_op: AssignOp, rhs: Expr) -> Self {
        AssignExpr {
            lhs,
            assign_op,
            rhs: Box::new(rhs),
        }
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

from_token! {
    #[derive(StrEnum, Debug, PartialEq)]
    pub enum AssignOp {
        /// Compound assignment operators
        #[strenum("+=")]
        PlusEq,

        #[strenum("-=")]
        MinusEq,

        #[strenum("*=")]
        StarEq,

        #[strenum("/=")]
        SlashEq,

        #[strenum("%=")]
        PercentEq,

        #[strenum("^=")]
        CaretEq,

        #[strenum("&=")]
        AndEq,

        #[strenum("|=")]
        OrEq,

        #[strenum("<<=")]
        ShlEq,

        #[strenum(">>=")]
        ShrEq,

        /// Assignment operators
        #[strenum("=")]
        Eq,
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeExpr {
    pub lhs: Option<Box<Expr>>,
    pub range_op: RangeOp,
    pub rhs: Option<Box<Expr>>,
}

impl RangeExpr {
    pub fn new(range_op: RangeOp) -> Self {
        RangeExpr {
            lhs: None,
            range_op,
            rhs: None,
        }
    }

    pub fn lhs(mut self, lhs: Expr) -> Self {
        self.set_lhs(lhs);
        self
    }

    pub fn rhs(mut self, rhs: Expr) -> Self {
        self.set_rhs(rhs);
        self
    }

    pub fn set_lhs(&mut self, lhs: Expr) {
        self.lhs = Some(Box::new(lhs));
    }

    pub fn set_rhs(&mut self, rhs: Expr) {
        self.rhs = Some(Box::new(rhs));
    }
}

impl TokenStart for RangeExpr {
    fn is_token_start(tk: &Token) -> bool {
        tk == &Token::DotDotEq || tk == &Token::DotDot
    }
}

from_token! {
    #[derive(StrEnum, Debug, PartialEq)]
    pub enum RangeOp {
        /// Range operators
        #[strenum("..")]
        DotDot,

        /// Range inclusive operators
        #[strenum("..=")]
        DotDotEq,
    }
}

#[derive(Debug, PartialEq)]
pub struct BinOpExpr {
    pub lhs: Box<Expr>,
    pub bin_op: BinOperator,
    pub rhs: Box<Expr>,
    pub type_info: TypeInfo,
}

impl BinOpExpr {
    pub fn new(lhs: Expr, bin_op: BinOperator, rhs: Expr) -> Self {
        BinOpExpr {
            lhs: Box::new(lhs),
            bin_op,
            rhs: Box::new(rhs),
            type_info: TypeInfo::Unknown,
        }
    }
}

impl ExprVisit for BinOpExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Place
    }
}

from_token! {
    #[derive(StrEnum, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum BinOperator {
        /// Arithmetic or logical operators
        #[strenum("+")]
        Plus,

        #[strenum("-")]
        Minus,

        #[strenum("*")]
        Star,

        #[strenum("/")]
        Slash,

        #[strenum("%")]
        Percent,

        #[strenum("^")]
        Caret,

        #[strenum("&")]
        And,

        #[strenum("|")]
        Or,

        #[strenum("<<")]
        Shl,

        #[strenum(">>")]
        Shr,

        /// Lazy boolean operators
        #[strenum("&&")]
        AndAnd,

        #[strenum("||")]
        OrOr,

        /// Type cast operator
        As,

        /// Comparison operators
        #[strenum("==")]
        EqEq,

        #[strenum("!=")]
        Ne,

        #[strenum(">")]
        Gt,

        #[strenum("<")]
        Lt,

        #[strenum(">=")]
        Ge,

        #[strenum("<=")]
        Le,
    }
}

impl Debug for BinOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

/// # Examples
///
/// ```
/// assert!(Precedence::As < Precedence::Multi);
/// ```
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Precedence {
    As,
    Multi,
    Add,
    Shift,
    And,
    Xor,
    Or,
    Cmp,
    AndAnd,
    OrOr,
}

impl Precedence {
    pub fn from_bin_op(op: &BinOperator) -> Self {
        match op {
            BinOperator::As => Self::As,
            BinOperator::Star | BinOperator::Slash | BinOperator::Percent => Self::Multi,
            BinOperator::Plus | BinOperator::Minus => Self::Add,
            BinOperator::Shl | BinOperator::Shr => Self::Shift,
            BinOperator::And => Self::And,
            BinOperator::Caret => Self::Xor,
            BinOperator::Or => Self::Or,
            BinOperator::EqEq
            | BinOperator::Ne
            | BinOperator::Gt
            | BinOperator::Lt
            | BinOperator::Ge
            | BinOperator::Le => Self::Cmp,
            BinOperator::AndAnd => Self::AndAnd,
            BinOperator::OrOr => Self::OrOr,
        }
    }
}

/// GroupExpr -> `(` Expr `)`
pub type GroupedExpr = Box<Expr>;

#[derive(Debug, PartialEq)]
pub struct ArrayExpr {
    pub elems: Vec<Expr>,
    pub len_expr: ConstantExpr<usize>,
}

impl ArrayExpr {
    pub fn new(elems: Vec<Expr>, len_expr: ConstantExpr<usize>) -> Self {
        ArrayExpr { elems, len_expr }
    }

    pub fn elems(elems: Vec<Expr>) -> ArrayExpr {
        let length = elems.len();
        ArrayExpr {
            elems,
            len_expr: ConstantExpr::<usize>::const_value(length),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ArrayIndexExpr {
    pub expr: Box<Expr>,
    pub index_expr: Box<Expr>,
}

impl ArrayIndexExpr {
    pub fn new(expr: Expr, index_expr: Expr) -> Self {
        ArrayIndexExpr {
            expr: Box::new(expr),
            index_expr: Box::new(index_expr),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TupleExpr(pub Vec<Expr>);

#[derive(Debug, PartialEq)]
pub struct TupleIndexExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct StructExpr;

#[derive(Debug, PartialEq)]
pub struct ReturnExpr(pub Option<Box<Expr>>);

impl ExprVisit for ReturnExpr {
    fn type_info(&self) -> TypeInfo {
        TypeInfo::Never
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

#[derive(Debug, PartialEq)]
pub struct BreakExpr(pub Option<Box<Expr>>);

impl ExprVisit for BreakExpr {
    fn type_info(&self) -> TypeInfo {
        TypeInfo::Never
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    pub expr: Box<Expr>,
    pub call_params: CallParams,
    pub type_info: TypeInfo,
}

pub type CallParams = Vec<Expr>;

impl CallExpr {
    pub fn new(expr: Expr) -> Self {
        CallExpr {
            expr: Box::new(expr),
            call_params: vec![],
            type_info: TypeInfo::Unknown,
        }
    }

    pub fn call_params(mut self, call_params: Vec<Expr>) -> Self {
        self.call_params = call_params;
        self
    }
}

impl ExprVisit for CallExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldAccessExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl FieldAccessExpr {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        FieldAccessExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfExpr {
    conditions: Vec<Expr>,
    blocks: Vec<BlockExpr>,
}

impl IfExpr {
    pub fn new() -> Self {
        IfExpr {
            conditions: vec![],
            blocks: vec![],
        }
    }

    pub fn from_exprs(conditions: Vec<Expr>, blocks: Vec<BlockExpr>) -> IfExpr {
        IfExpr { conditions, blocks }
    }

    pub fn add_cond(&mut self, expr: Expr) {
        self.conditions.push(expr);
    }

    pub fn add_block(&mut self, block_expr: BlockExpr) {
        self.blocks.push(block_expr);
    }
}

#[derive(Debug, PartialEq)]
pub struct WhileExpr(pub Box<Expr>, pub Box<BlockExpr>);

impl ExprVisit for WhileExpr {
    fn type_info(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}

#[derive(Debug, PartialEq)]
pub struct LoopExpr {
    pub expr: Box<BlockExpr>,
    pub type_info: TypeInfo,
}

impl LoopExpr {
    pub fn new(expr: BlockExpr) -> LoopExpr {
        LoopExpr {
            expr: Box::new(expr),
            type_info: TypeInfo::Unknown,
        }
    }
}

impl ExprVisit for LoopExpr {
    fn type_info(&self) -> TypeInfo {
        self.type_info.clone()
    }

    fn kind(&self) -> ExprKind {
        ExprKind::Value
    }
}
