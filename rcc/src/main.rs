use clap::Clap;
use crate::rcc::{RcCompiler, RccError};
use std::error::Error;
use std::str::FromStr;
use code_gen::TargetPlatform;

mod ast;
mod ir;
mod lexer;
mod parser;
mod rcc;
mod analyser;
mod code_gen;

#[derive(Clap)]
struct Opts {
    /// output asm file
    #[clap(short = 'S')]
    output_asm: bool,
    /// input file
    input: String,
    /// output file
    #[clap(short = 'o')]
    output: String,
    /// target platform
    #[clap(short = 't', default_value = "riscv32")]
    target: String,
}

fn compile(opts: Opts) -> Result<(), Box<dyn Error>> {
    match TargetPlatform::from_str(&opts.target) {
        Ok(target_platform) => {
            let input = std::fs::File::open(opts.input)?;
            let output = std::fs::File::create(opts.output)?;
            let mut rc_compiler = RcCompiler::new(target_platform, input, output);
            rc_compiler.compile()?;
            Ok(())
        }
        Err(_) => Err(Box::new(RccError(format!(
            "invalid target platform {}",
            opts.input
        )))),
    }
}

fn main() {
    let opts = Opts::parse();
    if let Err(e) = compile(opts) {
        eprintln!("{}", e);
    }
}
