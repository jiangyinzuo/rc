mod riscv32;
use crate::code_gen::riscv32::Riscv32CodeGen;
use strenum::EnumFromStr;

#[derive(EnumFromStr)]
pub enum TargetPlatform {
    Riscv32,
}

impl TargetPlatform {
    pub fn get_code_gen(&self) -> impl CodeGen {
        match self {
            Self::Riscv32 => Riscv32CodeGen::new(),
        }
    }
}

pub trait CodeGen {
    fn generate_code(&self) -> Result<(), String>;
}
