use crate::analyser::sym_resolver::SymbolResolver;
use crate::ast::AST;
use crate::code_gen::riscv32::Riscv32CodeGen;
use crate::code_gen::TargetPlatform;
use crate::ir::cfg::CFGIR;
use crate::ir::ir_build::IRBuilder;
use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Copy, Clone)]
pub enum OptimizeLevel {
    Zero,
    One,
}

pub struct RcCompiler<R: Read, W: Write> {
    input: BufReader<R>,
    pub output: BufWriter<W>,
    opt_level: OptimizeLevel,
}

impl<R: Read, W: Write> RcCompiler<R, W> {
    pub fn new(
        target_platform: TargetPlatform,
        input: R,
        output: W,
        opt_level: OptimizeLevel,
    ) -> Self {
        RcCompiler {
            input: BufReader::new(input),
            output: BufWriter::new(output),
            opt_level,
        }
    }

    pub fn compile(&mut self) -> Result<(), RccError> {
        let mut input = String::new();
        self.input.read_to_string(&mut input)?;

        // lex
        let mut lexer = Lexer::new(input.as_str());
        let token_stream = lexer.tokenize();

        // parse
        let mut cursor = ParseCursor::new(token_stream);
        let mut ast = AST::parse(&mut cursor)?;

        let mut sym_resolver = SymbolResolver::new();
        sym_resolver.visit_file(&mut ast.file)?;

        let mut ir_builder = IRBuilder::new(self.opt_level);
        let linear_ir = ir_builder.generate_ir(&mut ast)?;

        let cfg_ir = CFGIR::new(linear_ir);
        cfg_ir.reaching_definitions_analysis()?;

        match self.opt_level {
            OptimizeLevel::Zero => {
                let mut code_gen = Riscv32CodeGen::new(cfg_ir, &mut self.output, self.opt_level);
                code_gen.run()?;
            }
            OptimizeLevel::One => {
                todo!()
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RccError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("{0}")]
    Parse(String),
}

impl From<String> for RccError {
    fn from(s: String) -> Self {
        RccError::Parse(s)
    }
}

impl From<&str> for RccError {
    fn from(s: &str) -> Self {
        RccError::Parse(s.to_string())
    }
}

impl PartialEq for RccError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RccError::IO(e) => {
                if let RccError::IO(o) = other {
                    return e.to_string() == o.to_string();
                }
                false
            }
            RccError::Parse(s) => {
                if let RccError::Parse(o) = other {
                    return s == o;
                }
                false
            }
            RccError::ParseInt(p) => {
                if let RccError::ParseInt(o) = other {
                    return p == o;
                }
                false
            }
            RccError::ParseFloat(p) => {
                if let RccError::ParseFloat(o) = other {
                    return p == o;
                }
                false
            }
        }
    }
}
