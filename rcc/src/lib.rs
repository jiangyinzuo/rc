use crate::code_gen::{TargetPlatform, CodeGen};
use crate::lexer::Lexer;
use crate::parser::{ParseContext, Parse};
use crate::ast::file::File;
use crate::ir::{IRGenContext, IRGen};
use std::ops::Deref;

mod ir;
mod parser;
mod ast;
mod code_gen;
mod lexer;

pub struct RcCompiler {
    code_gen: Box<dyn CodeGen>,
}

impl RcCompiler {
    pub fn new(target_platform: TargetPlatform) -> Self {
        let code_gen = target_platform.get_code_gen();
        RcCompiler {code_gen: Box::new(code_gen)}
    }

    pub fn compile(&self, input: String) -> Result<(), String> {
        let mut lexer = Lexer::new(input.as_str());
        let token_stream = lexer.tokenize();
        let mut cxt = ParseContext::new(token_stream);
        let ast_file = File::parse(&mut cxt)?;
        let mut ir_gen_cxt = IRGenContext::new();
        ast_file.generate(&mut ir_gen_cxt)?;
        self.code_gen.deref().generate_code()?;
        Ok(())
    }
}