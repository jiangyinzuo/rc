//! Notation        | Examples        | Meaning
//! -----------------------------------------------------------------
//! snake_case `x`  | fn, `|`         | A token produced by the lexer
//! ItalicCamelCase | VisItem, Item   | A syntactical production
//! x?              | pub?            | An optional item
//! x*              | OuterAttribute* | 0 or more of x
//! x+              | MacroMatch+     | 1 or more of x
//! |               | u8 | u16, Block | Item Either one or another
//! ( )             | (, Parameter)?  | Groups items
//! -----------------------------------------------------------------
//!
//! `Syntactical Productions:`
//!
//! File -> Item File | Item
//! Item -> pub? VisItem | Impl
//! VisItem -> Fn | Struct | Enum | Const | Static
//!
//! Fn -> FnSignature BlockExpr
//! FnSignature -> fn Ident `(` FnArgs? `)` RetType?
//! FnArgs -> FnArg (`,` FnArg)* `,`?
//! RetType -> r_arrow Type
//! Type -> ident | `()` | ( left_paren (Type comma)+ Type? right_paren ) |
//!         bool | char |
//!         f32 | f64 | i8 | i16 | i32 | i64 |
//!         i128 | isize | u8 | u16 | u32 | u64 | u128 | usize
//!
//! Static -> static ident TypeAnnotation eq semi

use std::fmt::Debug;

use crate::lexer::token::Token;

pub mod expr;
pub mod file;
pub mod item;
pub mod type_anno;

#[cfg(test)]
mod tests;

pub enum Visibility {
    Pub,
    Priv,
}

pub trait Parse<'a>: Sized + Debug + PartialEq {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str>;
}

#[derive(Clone)]
pub struct ParseContext<'a> {
    token_stream: Vec<Token<'a>>,
    token_idx: usize,
}

impl<'a> ParseContext<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        ParseContext {
            token_stream,
            token_idx: 0,
        }
    }

    pub fn next_token(&self) -> Result<&Token<'a>, &'static str> {
        match self.token_stream.get(self.token_idx) {
            Some(tk) => Ok(tk),
            None => Err("EOF token")
        }
    }

    pub fn bump_token(&mut self) -> Result<&Token<'a>, &'static str> {
        match self.token_stream.get(self.token_idx) {
            Some(tk) => {
                self.token_idx += 1;
                Ok(tk)
            }
            None => Err("EOF token")
        }
    }

    pub fn is_eof(&self) -> bool {
        self.token_idx == self.token_stream.len()
    }
}
