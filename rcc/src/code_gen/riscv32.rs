use crate::code_gen::CodeGen;

pub struct Riscv32CodeGen {}

impl Riscv32CodeGen {
    pub fn new() -> Self {
        Riscv32CodeGen {}
    }
}

impl CodeGen for Riscv32CodeGen {
    fn generate_code(&self) -> Result<(), String> {
        unimplemented!()
    }
}