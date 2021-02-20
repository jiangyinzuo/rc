#![feature(map_first_last)]

use std::str::FromStr;
use clap::Clap;
use code_gen::TargetPlatform;
use crate::rcc::{OptimizeLevel, RccError, RcCompiler};

mod analyser;
mod ast;
mod code_gen;
mod ir;
mod lexer;
mod parser;
mod rcc;
mod tests;

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

fn compile(opts: Opts) -> Result<(), RccError> {
    match TargetPlatform::from_str(&opts.target) {
        Ok(target_platform) => {
            let input = std::fs::File::open(opts.input)?;
            let output = std::fs::File::create(opts.output)?;
            // TODO: set opt level
            let mut rc_compiler =
                RcCompiler::new(target_platform, input, output, OptimizeLevel::Zero);
            rc_compiler.compile()?;
            Ok(())
        }
        Err(_) => Err(format!("invalid target platform {}", opts.input).into()),
    }
}

fn main() {
    let opts = Opts::parse();
    if let Err(e) = compile(opts) {
        eprintln!("{:?}", e);
    }
}
