use strenum::EnumFromStr;
use crate::token::LiteralKind::{Integer, Float};

#[derive(Clone, Debug, PartialEq, EnumFromStr)]
pub enum Token<'a> {
    /// Strict keywords
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mut,
    Ref,
    Return,

    #[value("self")]
    SelfValue,

    #[value("Self")]
    SelfType,

    Static,
    Struct,
    True,
    While,

    /// Reserved keywords
    Crate,
    Mod,
    Move,
    Super,
    Trait,
    Type,
    Unsafe,
    Use,
    Where,
    Async,
    Await,
    Dyn,
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,
    Try,
    Union,

    /// Primitive types(i8, bool etc.) are seen as identifiers at this period
    #[disabled]
    Identifier(&'a str),

    /// Literals
    #[disabled]
    Literal {
        literal_kind: LiteralKind<'a>,
        value: &'a str,
    },

    /// Symbols
    #[value("+")]
    Plus,

    #[value("-")]
    Minus,

    #[value("*")]
    Star,

    #[value("/")]
    Slash,

    #[value("%")]
    Percent,

    #[value("^")]
    Caret,

    #[value("!")]
    Not,

    #[value("&")]
    And,

    #[value("|")]
    Or,

    #[value("&&")]
    AndAnd,

    #[value("||")]
    OrOr,

    #[value("<<")]
    Shl,

    #[value(">>")]
    Shr,

    #[value("+=")]
    PlusEq,

    #[value("-=")]
    MinusEq,

    #[value("*=")]
    StarEq,

    #[value("/=")]
    SlashEq,

    #[value("%=")]
    PercentEq,

    #[value("^=")]
    CaretEq,

    #[value("&=")]
    AndEq,

    #[value("|=")]
    OrEq,

    #[value("<<=")]
    ShlEq,

    #[value(">>=")]
    ShrEq,

    #[value("=")]
    Eq,

    #[value("==")]
    EqEq,

    #[value("!=")]
    Ne,

    #[value(">")]
    Gt,

    #[value("<")]
    Lt,

    #[value(">=")]
    Ge,

    #[value("<=")]
    Le,

    #[value("@")]
    At,

    #[value("_")]
    Underscore,

    #[value(".")]
    Dot,

    #[value("..")]
    DotDot,

    #[value("...")]
    DotDotDot,

    #[value("..=")]
    DotDotEq,

    #[value(",")]
    Comma,

    #[value(";")]
    Semi,

    #[value(":")]
    Colon,

    #[value("::")]
    PathSep,

    #[value("->")]
    RArrow,

    #[value("=>")]
    FatArrow,

    #[value("#")]
    Pound,

    #[value("$")]
    Dollar,

    #[value("?")]
    Question,

    /// delimiters
    #[value("{")]
    LeftCurlyBraces,

    #[value("}")]
    RightCurlyBraces,

    #[value("[")]
    LeftSquareBrackets,

    #[value("]")]
    RightSquareBrackets,

    #[value("(")]
    LeftParen,

    #[value(")")]
    RightParen,

    #[disabled]
    WhiteSpace,

    #[disabled]
    Comment,

    #[disabled]
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind<'a> {
    Integer {
        suffix: &'a str
    },
    Char,
    Float {
        suffix: &'a str
    },
    String,
}

impl<'a> LiteralKind<'a> {
    pub const fn integer_no_suffix() -> LiteralKind<'a> {
        Integer { suffix: "" }
    }

    pub const fn float_no_suffix() -> LiteralKind<'a> {
        Float {suffix: ""}
    }
}
