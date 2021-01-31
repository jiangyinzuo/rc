use crate::lexer::Lexer;
use crate::parser::{ParseCursor, Parse};
use crate::rcc::RccError;
use crate::ast::file::File;

mod sym_resolver_tests;
mod scope_test;

fn get_ast_file(input: &str) -> Result<File, RccError> {
    // lex
    let mut lexer = Lexer::new(input);
    let token_stream = lexer.tokenize();

    // parse
    let mut cursor = ParseCursor::new(token_stream);
    let ast_file = File::parse(&mut cursor)?;
    Ok(ast_file)
}