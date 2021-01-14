use strenum::EnumFromStr;
use std::str::FromStr;
use crate::token::TokenKind::{Plus, Comment};

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, PartialEq, EnumFromStr)]
pub enum TokenKind {
    /// strict keywords
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
    SelfValue,
    SelfType,
    Static,
    Struct,
    True,
    While,

    /// reserved keywords
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

    #[disabled]
    Identifier,

    #[disabled]
    Literals,

    /// symbols
    #[value("+")]
    Plus,

    Minus,

    Star,

    Slash,

    Percent,

    Caret,

    Not,

    And,

    Or,

    AndAnd,

    OrOr,

    Shl,

    Shr,

    PlusEq,

    MinusEq,

    StarEq,

    SlashEq,

    PercentEq,

    CaretEq,

    ShlEq,

    ShrEq,

    Eq,

    EqEq,

    Ne,

    Gt,

    Lt,

    Ge,

    Le,

    At,

    Underscore,

    Dot,

    DotDot,

    DotDotDot,

    DotDotEq,

    Comma,

    Semi,

    Colon,

    PathSep,

    RArrow,

    FatArrow,

    Pound,

    Dollar,

    Question,

    /// delimiters

    LeftCurlyBraces,

    RightCurlyBraces,

    LeftSquareBrackets,

    RightSquareBrackets,

    LeftParentheses,

    RightParentheses,

    WhiteSpace,

    Comment,

    Unknown,
}

#[test]
fn token_kind_test() {
    let a = TokenKind::from_str("comment").unwrap();
    assert_eq!(Comment, a);
    let plus = TokenKind::from_str("+").unwrap();
    assert_eq!(Plus, plus);
}