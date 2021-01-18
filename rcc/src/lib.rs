use lexer::token::Token;
use std::result::Result;
mod parser;

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

    pub fn next_token(&self) -> Option<&Token<'a>> {
        self.token_stream.get(self.token_idx)
    }

    pub fn bump_token(&mut self) -> Option<&Token<'a>> {
        let next_token = self.token_stream.get(self.token_idx);
        self.token_idx += 1;
        next_token
    }
}

pub trait Parse<'a>: Sized {
    fn parse(cxt: &mut ParseContext<'a>) -> Result<Self, &'static str>;
}
