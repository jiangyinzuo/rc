pub mod ir_gen;

pub enum Opcode {
    Ret
}

pub struct Quad {
    op: Opcode,
    src1: String,
    src2: String,
}

pub struct BaseBlock {
    name: String,
    quads: Vec<Quad>,
}

pub trait IRGen<T> {
    fn generate(&mut self, cxt: &mut IRGenContext) -> Result<T, &str>;
}

pub struct IRGenContext;