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

use crate::ast::{Visibility, AST};
use crate::lexer::token::{LiteralKind, Token};
use crate::rcc::RccError;
use crate::ast::{FromToken};

pub mod expr;
pub mod file;
pub mod item;

#[cfg(test)]
mod tests;
mod types;
mod stmt;
mod pattern;

pub trait Parse: Sized + Debug + PartialEq {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError>;
}

#[derive(Clone)]
pub struct ParseCursor<'a> {
    token_stream: Vec<Token<'a>>,
    token_idx: usize,
}

impl<'a> ParseCursor<'a> {
    pub fn new(token_stream: Vec<Token<'a>>) -> Self {
        ParseCursor {
            token_stream,
            token_idx: 0,
        }
    }

    pub fn next_token(&self) -> Result<&Token<'a>, RccError> {
        match self.token_stream.get(self.token_idx) {
            Some(tk) => Ok(tk),
            None => Err("EOF token".into()),
        }
    }

    pub fn bump_token(&mut self) -> Result<&Token<'a>, RccError> {
        match self.token_stream.get(self.token_idx) {
            Some(tk) => {
                self.token_idx += 1;
                Ok(tk)
            }
            None => Err("EOF token".into()),
        }
    }

    pub fn eat_identifier(&mut self) -> Result<&'a str, RccError> {
        match self.bump_token()? {
            Token::Identifier(s) => Ok(s),
            _ => Err(self.err("identifier".to_string()).into()),
        }
    }

    pub fn eat_literal(&mut self) -> Result<(LiteralKind, String), RccError> {
        match self.bump_token()? {
            Token::Literal {
                literal_kind,
                value,
            } => Ok((literal_kind.clone(), value.to_string())),
            _ => Err(self.err("literal".to_string()).into()),
        }
    }

    pub fn eat_token_eq(&mut self, tk: Token) -> Result<(), RccError> {
        if self.bump_token()? != &tk {
            Err(self.err(tk.to_string()).into())
        } else {
            Ok(())
        }
    }

    pub fn eat_token_in(&mut self, tks: &[Token]) -> Result<&Token, RccError> {
        let next_token = self.next_token()?;
        for tk in tks {
            if next_token == tk {
                return Ok(self.bump_token()?);
            }
        }
        Err(self.err(format!("{:?}", tks)).into())
    }

    pub fn eat_token_if_eq(&mut self, tk: Token) -> bool {
        if let Ok(next_tk) = self.next_token() {
            if next_tk == &tk {
                self.bump_token();
                return true;
            }
        }
        false
    }

    pub fn eat_token_if_in(&mut self, tks: &[Token]) -> Option<&Token> {
        for tk in tks {
            if let Ok(next_tk) = self.next_token() {
                if next_tk == tk {
                    return Some(self.bump_token().unwrap());
                }
            }
        }
        None
    }

    pub fn eat_token_if_from<T: FromToken>(&mut self) -> Option<T> {
        if let Ok(tk) = self.next_token() {
            let op = T::from_token(tk.clone());
            if op.is_some() && self.bump_token().is_err() {
                return None;
            }
            op
        } else {
            None
        }
    }

    fn err(&self, expect: String) -> String {
        format!("error in parsing: except {}", expect)
    }

    pub fn is_eof(&self) -> bool {
        self.token_idx == self.token_stream.len()
    }
}

impl Parse for Visibility {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        match cursor.next_token()? {
            Token::Pub => {
                cursor.bump_token()?;
                Ok(Visibility::Pub)
            }
            _ => Ok(Visibility::Priv),
        }
    }
}

impl Parse for AST {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let file = crate::ast::file::File::parse(cursor)?;
        Ok(AST {
            file
        })
    }
}
