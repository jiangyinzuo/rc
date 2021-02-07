use crate::ast::file::File;
use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufReader, BufWriter, Read, Write};
use crate::code_gen::TargetPlatform;

pub struct RcCompiler<R: Read, W: Write> {
    input: BufReader<R>,
    output: BufWriter<W>,
}

impl<R: Read, W: Write> RcCompiler<R, W> {
    pub fn new(target_platform: TargetPlatform, input: R, output: W) -> Self {
        RcCompiler {
            input: BufReader::new(input),
            output: BufWriter::new(output),
        }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        self.input.read_to_string(&mut input)?;

        // lex
        let mut lexer = Lexer::new(input.as_str());
        let token_stream = lexer.tokenize();

        // parse
        let mut cursor = ParseCursor::new(token_stream);
        let ast_file = File::parse(&mut cursor)?;

        // TODO semantic analyser, generate IR
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct RccError(pub String);

impl Display for RccError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl Error for RccError {}

impl From<String> for RccError {
    fn from(s: String) -> Self {
        RccError(s)
    }
}

impl From<&str> for RccError {
    fn from(s: &str) -> Self {
        RccError(s.to_string())
    }
}
