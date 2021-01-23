use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

mod cursor_test;
mod expr_tests;
mod item_tests;
mod file_tests;

fn get_parser(input: &str) -> ParseCursor {
    let mut lexer = Lexer::new(input);
    ParseCursor::new(lexer.tokenize())
}

fn parse_input<'a, T: Parse<'a>>(input: &'a str) -> Result<T, RccError> {
    let mut lexer = Lexer::new(input);
    let mut cxt = ParseCursor::new(lexer.tokenize());
    T::parse(&mut cxt)
}

fn parse_validate<'a, T: Parse<'a>>(
    inputs: std::vec::Vec<&'a str>,
    excepted_segments: Vec<Result<T, &'static str>>,
) {
    for (input, excepted) in inputs.into_iter().zip(excepted_segments) {
        let result = parse_input::<T>(input);
        match excepted {
            Ok(segments) => assert_eq!(Ok(segments), result),
            Err(s) => assert_eq!(excepted.unwrap_err(), s),
        }
    }
}
