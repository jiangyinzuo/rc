use crate::lexer::Lexer;
use crate::parser::{ParseCursor, Parse};
use crate::rcc::RccError;

mod cursor_test;
mod expr_tests;

fn get_parser(input: &str) -> ParseCursor {
    let mut lexer = Lexer::new(input);
    ParseCursor::new(lexer.tokenize())
}

fn parse_input<'a, T: Parse<'a>>(input: &'a str) -> Result<T, RccError> {
    let mut lexer = Lexer::new(input);
    let mut cxt = ParseCursor::new(lexer.tokenize());
    T::parse(&mut cxt)
}
