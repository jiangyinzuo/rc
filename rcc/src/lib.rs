use lexer::token::Token;
use std::result::Result;
use std::fmt::Debug;

mod ir;
mod parser;

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

pub trait Parse<'a>: Sized + Debug + PartialEq {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str>;
}
