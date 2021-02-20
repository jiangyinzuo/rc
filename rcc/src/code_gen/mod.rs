pub mod code_generator;

use strenum::StrEnum;

#[derive(StrEnum)]
pub enum TargetPlatform {
    Riscv32
}