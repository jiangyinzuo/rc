use lexer::token::Token;
use std::result::Result;
mod parser;

pub struct ParseContext<'a> {
    token_stream: Vec<Token<'a>>,
}

pub trait Parse<'a>: Sized {
    fn parse(cxt: ParseContext<'a>) -> Result<Self, ()>;
}
