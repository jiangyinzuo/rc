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