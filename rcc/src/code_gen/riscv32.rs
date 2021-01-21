use crate::code_gen::CodeGen;
use std::io::{Write};
use crate::ir::BasicBlock;

pub struct Riscv32CodeGen {}

impl Riscv32CodeGen {
    pub fn new() -> Self {
        Riscv32CodeGen {}
    }
}

impl CodeGen for Riscv32CodeGen {
    fn generate_code(&self, write: &mut dyn Write, basic_blocks: Vec<BasicBlock>) -> Result<(), String> where Self: Sized {
        let s = "hello".as_bytes();
        for basic_block in basic_blocks {
            write.write(s);
        }
        Ok(())
    }
}