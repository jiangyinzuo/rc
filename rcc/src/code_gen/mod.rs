pub mod code_generator;
pub(crate) mod simple_allocator;

use strenum::StrEnum;

#[derive(StrEnum)]
pub enum TargetPlatform {
    Riscv32
}