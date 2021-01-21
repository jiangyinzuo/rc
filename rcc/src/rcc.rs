use crate::ast::file::File;
use crate::code_gen::{CodeGen, TargetPlatform};
use crate::ir::{IRGen, IRGenContext, BasicBlock};
use crate::lexer::Lexer;
use crate::parser::{Parse, ParseContext};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct RcCompiler<R: Read, W: Write> {
    code_gen: Box<dyn CodeGen>,
    input: BufReader<R>,
    output: BufWriter<W>,
}

impl <R: Read, W: Write> RcCompiler<R, W> {
    pub fn new(
        target_platform: TargetPlatform,
        input: R,
        output: W,
    ) -> Self {
        let code_gen = target_platform.get_code_gen();
        RcCompiler {
            code_gen: Box::new(code_gen),
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
        let mut cxt = ParseContext::new(token_stream);
        let ast_file = File::parse(&mut cxt)?;

        // generate ir
        let mut ir_gen_cxt = IRGenContext::new();
        ast_file.generate(&mut ir_gen_cxt)?;
        // generate target asm
        self.code_gen.deref().generate_code(&mut self.output, ir_gen_cxt.basic_blocks)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RccError(pub String);

impl Display for RccError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl Error for RccError {}
