use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use strenum::StrEnum;

use crate::ast::stmt::Stmt;
use crate::ast::TokenStart;
use crate::lexer::token::Token;
use crate::lexer::token::Token::{Minus, Not, Star};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Path(PathExpr),
    Lit(LitExpr),
    Unary(UnAryExpr),
    Block(BlockExpr),
    Borrow(BorrowExpr),
    Assign(AssignExpr),
    Range(RangeExpr),
    BinOp(BinOpExpr),
    Group(GroupExpr),
    Array(ArrayExpr),
    ArrayIndex(ArrayIndexExpr),
    Tuple(TupleExpr),
    TupleIndex(TupleIndexExpr),
    Struct(StructExpr),
    EnumVariant,
    Call,
    MethodCall,
    FieldAccess,
    Loop(LoopExpr),
    If,
    Match,
    Return(ReturnExpr),
}

impl TokenStart for Expr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk,
            Token::Identifier(_) | Token::Literal {..} | Token::DotDot |
            Token::LeftCurlyBraces | Token::LeftParen | Token::LeftSquareBrackets |
            Token::If | Token::Match | Token::Return
        ) || UnAryExpr::is_token_start(tk)
            || BorrowExpr::is_token_start(tk)
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
        BlockExpr {
            stmts
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LitExpr {
    pub ret_type: String,
    pub value: String,
}

impl LitExpr {
    pub fn lit_i32(value: &str) -> Self {
        LitExpr {
            ret_type: "i32".into(),
            value: value.into(),
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

impl TokenStart for UnAryExpr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::Not | Token::Star | Token::Minus)
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
}

impl Debug for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::Deref => '*',
            Self::Not => '!',
            Self::Neg => '-',
        })
    }
}

impl UnOp {
    pub fn from_token(tk: &Token) -> Option<Self> {
        match tk {
            Minus => Some(Self::Neg),
            Star => Some(Self::Deref),
            Not => Some(Self::Not),
            _ => None,
        }
    }
}

/// BorrowExpr -> (& | &&) mut? Expr
#[derive(Debug, PartialEq)]
pub struct BorrowExpr {
    pub borrow_cnt: u32,
    pub is_mut: bool,
    pub expr: Box<Expr>,
}

impl TokenStart for BorrowExpr {
    fn is_token_start(tk: &Token) -> bool {
        tk == &Token::And || tk == &Token::AndAnd
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
        self.lhs = Some(Box::new(lhs));
        self
    }

    pub fn rhs(mut self, rhs: Expr) -> Self {
        self.rhs = Some(Box::new(rhs));
        self
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
    bin_op: BinOp,
    rhs: Box<Expr>,
}

#[derive(StrEnum, Debug, PartialEq)]
pub enum BinOp {
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

/// GroupExpr -> `(` Expr `)`
#[derive(Debug, PartialEq)]
pub struct GroupExpr(Box<Expr>);

#[derive(Debug, PartialEq)]
pub struct ArrayExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct ArrayIndexExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct TupleExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct TupleIndexExpr {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct StructExpr;

#[derive(Debug, PartialEq)]
pub struct ReturnExpr(pub Box<Expr>);

#[derive(Debug, PartialEq)]
pub struct LoopExpr;

impl TokenStart for LoopExpr {
    fn is_token_start(tk: &Token) -> bool {
        matches!(tk, Token::For | Token::Loop | Token::While)
    }
}