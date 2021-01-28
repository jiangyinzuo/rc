use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use strenum::StrEnum;
use crate::ast::stmt::Stmt;
use crate::ast::TokenStart;
use crate::lexer::token::LiteralKind::{Char, Integer};
use crate::lexer::token::Token;
use crate::lexer::token::Token::{Minus, Not, Star};
use std::cmp::Ordering;

macro_rules! from_token {
    (
        #[$($attrs_pub:tt)*]
        pub enum $name:ident {
            $(
              $(#[$($attrs:tt)*])*
              $variant:ident,)*
        }
    ) => {
        #[$($attrs_pub)*]
        pub enum $name {
            $(
              $(#[$($attrs)*])*
              $variant,)*
        }

       impl FromToken for BinOperator {
           fn from_token(tk: Token) -> Option<Self> {
               match tk {
                   $(Token::$variant => Some(Self::$variant),)*
                   _ => None,
               }
           }
       }
    };
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Path(PathExpr),
    Lit(LitExpr),
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
    Loop(LoopExpr),
    If,
    Match,
    Return(ReturnExpr),
    Break(BreakExpr),
}

#[derive(Debug, PartialEq)]
pub struct ConstantExpr<V> {
    expr: Option<Box<Expr>>,
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
            Token::Identifier(_) | Token::Literal {..} | Token::DotDot |
            Token::LeftCurlyBraces | Token::LeftParen | Token::LeftSquareBrackets |
            Token::If | Token::Match | Token::Return
        ) || UnAryExpr::is_token_start(tk)
            || RangeExpr::is_token_start(tk)
            || LoopExpr::is_token_start(tk)
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
}

impl BlockExpr {
    pub fn new() -> Self {
        BlockExpr { stmts: vec![] }
    }
}

impl From<Vec<Stmt>> for BlockExpr {
    fn from(stmts: Vec<Stmt>) -> Self {
        BlockExpr { stmts }
    }
}

#[derive(PartialEq, Debug)]
pub struct LitExpr {
    pub ret_type: String,
    pub value: String,
}

impl LitExpr {
    pub const EMPTY_INT_TYPE: &'static str = "#i";
    pub const EMPTY_FLOAT_TYPE: &'static str = "#f";
}

impl From<i32> for LitExpr {
    fn from(num: i32) -> Self {
        LitExpr {
            ret_type: Self::EMPTY_INT_TYPE.to_string(),
            value: num.to_string(),
        }
    }
}

impl From<char> for LitExpr {
    fn from(c: char) -> Self {
        LitExpr {
            ret_type: "char".to_string(),
            value: format!("'{}'", c),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PathExpr {
    pub segments: Vec<String>,
}

impl PathExpr {
    pub fn new() -> Self {
        PathExpr { segments: vec![] }
    }
}

impl From<Vec<String>> for PathExpr {
    fn from(segments: Vec<String>) -> Self {
        PathExpr { segments }
    }
}

impl From<Vec<&str>> for PathExpr {
    fn from(segments: Vec<&str>) -> Self {
        PathExpr {
            segments: segments.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl From<&str> for PathExpr {
    fn from(s: &str) -> Self {
        PathExpr {
            segments: s.split("::").map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnAryExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl UnAryExpr {
    pub fn new(op: UnOp, expr: Expr) -> Self {
        UnAryExpr {
            op,
            expr: Box::new(expr),
        }
    }
}

impl TokenStart for UnAryExpr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::Not | Token::Star | Token::Minus | Token::And | Token::AndAnd)
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

pub trait FromToken: Sized {
    fn from_token(tk: Token) -> Option<Self>;
}

#[derive(Debug, PartialEq)]
pub struct AssignExpr {
    pub lhs: Box<Expr>,
    pub assign_op: AssignOp,
    pub rhs: Box<Expr>,
}

impl AssignExpr {
    pub fn new(lhs: Expr, assign_op: AssignOp, rhs: Expr) -> Self {
        AssignExpr {
            lhs: Box::new(lhs),
            assign_op,
            rhs: Box::new(rhs),
        }
    }
}

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

impl FromToken for AssignOp {
    fn from_token(tk: Token) -> Option<Self> {
        match tk {
            Token::PlusEq => Some(Self::PlusEq),
            Token::MinusEq => Some(Self::MinusEq),
            Token::StarEq => Some(Self::StarEq),
            Token::SlashEq => Some(Self::SlashEq),
            Token::PercentEq => Some(Self::PercentEq),
            Token::CaretEq => Some(Self::CaretEq),
            Token::AndEq => Some(Self::AndEq),
            Token::OrEq => Some(Self::OrEq),
            Token::ShlEq => Some(Self::ShlEq),
            Token::ShrEq => Some(Self::ShrEq),
            Token::Eq => Some(Self::Eq),
            _ => None,
        }
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

#[derive(StrEnum, Debug, PartialEq)]
pub enum RangeOp {
    /// Range operators
    #[strenum("..")]
    DotDot,

    /// Range inclusive operators
    #[strenum("..=")]
    DotDotEq,
}

impl FromToken for RangeOp {
    fn from_token(tk: Token) -> Option<Self> {
        match tk {
            Token::DotDot => Some(Self::DotDot),
            Token::DotDotEq => Some(Self::DotDotEq),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinOpExpr {
    lhs: Box<Expr>,
    bin_op: BinOperator,
    rhs: Box<Expr>,
}

impl BinOpExpr {
    pub fn new(lhs: Expr, bin_op: BinOperator, rhs: Expr) -> Self {
        BinOpExpr {
            lhs: Box::new(lhs),
            bin_op,
            rhs: Box::new(rhs),
        }
    }
}

from_token! {
#[derive(StrEnum, Debug, PartialEq, Clone)]
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

/// ```
/// assert!(Precedence::As < Precedence::Multi);
/// ```
#[derive(Debug, PartialOrd, PartialEq)]
enum Precedence {
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
    fn from_bin_op(op: &BinOperator) -> Self {
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

impl PartialOrd for BinOperator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Precedence::from_bin_op(self).partial_cmp(&Precedence::from_bin_op(other))
    }
}

/// GroupExpr -> `(` Expr `)`
pub type GroupedExpr = Box<Expr>;

#[derive(Debug, PartialEq)]
pub struct ArrayExpr {
    elems: Vec<Expr>,
    len: ConstantExpr<usize>,
}

impl ArrayExpr {
    pub fn new(elems: Vec<Expr>, len: ConstantExpr<usize>) -> Self {
        ArrayExpr { elems, len }
    }

    pub fn elems(elems: Vec<Expr>) -> Self {
        let length = elems.len();
        ArrayExpr {
            elems,
            len: ConstantExpr::<usize>::const_value(length),
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
pub struct ReturnExpr(pub Box<Expr>);

#[derive(Debug, PartialEq)]
pub struct BreakExpr(pub Option<Box<Expr>>);

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    pub expr: Box<Expr>,
    pub call_params: CallParams,
}

pub type CallParams = Vec<Expr>;

impl CallExpr {
    pub fn new(expr: Expr) -> Self {
        CallExpr {
            expr: Box::new(expr),
            call_params: vec![],
        }
    }

    pub fn call_params(mut self, call_params: Vec<Expr>) -> Self {
        self.call_params = call_params;
        self
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
pub struct LoopExpr;

impl TokenStart for LoopExpr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::For | Token::Loop | Token::While)
    }
}
