use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

mod cursor_test;
mod expr_tests;
mod item_tests;
mod file_tests;
mod stmt_tests;

fn get_parser(input: &str) -> ParseCursor {
    let mut lexer = Lexer::new(input);
    ParseCursor::new(lexer.tokenize())
}

fn parse_input<T: Parse>(input: &str) -> Result<T, RccError> {
    let mut lexer = Lexer::new(input);
    let mut cxt = ParseCursor::new(lexer.tokenize());
    T::parse(&mut cxt)
}

fn parse_validate<T: Parse>(
    inputs: std::vec::Vec<&str>,
    excepted_segments: Vec<Result<T, &str>>,
) {
    for (input, excepted) in inputs.into_iter().zip(excepted_segments) {
        let result = parse_input::<T>(input);
        match excepted {
            Ok(segments) => assert_eq!(Ok(segments), result),
            Err(s) => {
                assert_eq!(result.unwrap_err().0, s)
            },
        }
    }
}
