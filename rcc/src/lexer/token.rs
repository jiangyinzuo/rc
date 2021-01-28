use crate::lexer::token::LiteralKind::{Float, Integer};
use crate::lexer::token::Token::True;
use strenum::StrEnum;

#[derive(Clone, Debug, PartialEq, StrEnum)]
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
    Pub,
    Ref,
    Return,

    #[strenum("self")]
    SelfValue,

    #[strenum("Self")]
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
    #[strenum(disabled)]
    Identifier(&'a str),

    /// Literals
    #[strenum(disabled)]
    Literal {
        literal_kind: LiteralKind<'a>,
        value: &'a str,
    },

    /// Symbols
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

    #[strenum("!")]
    Not,

    #[strenum("&")]
    And,

    #[strenum("|")]
    Or,

    #[strenum("&&")]
    AndAnd,

    #[strenum("||")]
    OrOr,

    #[strenum("<<")]
    Shl,

    #[strenum(">>")]
    Shr,

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

    #[strenum("=")]
    Eq,

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

    #[strenum("@")]
    At,

    #[strenum(".")]
    Dot,

    #[strenum("..")]
    DotDot,

    #[strenum("...")]
    DotDotDot,

    #[strenum("..=")]
    DotDotEq,

    #[strenum(",")]
    Comma,

    #[strenum(";")]
    Semi,

    #[strenum(":")]
    Colon,

    #[strenum("::")]
    PathSep,

    #[strenum("->")]
    RArrow,

    #[strenum("=>")]
    FatArrow,

    #[strenum("#")]
    Pound,

    #[strenum("$")]
    Dollar,

    #[strenum("?")]
    Question,

    /// delimiters
    #[strenum("{")]
    LeftCurlyBraces,

    #[strenum("}")]
    RightCurlyBraces,

    #[strenum("[")]
    LeftSquareBrackets,

    #[strenum("]")]
    RightSquareBrackets,

    #[strenum("(")]
    LeftParen,

    #[strenum(")")]
    RightParen,

    #[strenum(disabled)]
    WhiteSpace,

    #[strenum(disabled)]
    Comment,

    #[strenum(disabled)]
    Unknown,
}

impl Token<'_> {
    pub fn is_range_op(&self) -> bool {
        matches!(self, Self::DotDot | Self::DotDotEq)
    }
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
