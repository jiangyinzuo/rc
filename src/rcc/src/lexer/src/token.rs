#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, PartialEq)]
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

    Identifier,

    Literals,

    /// symbols

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
