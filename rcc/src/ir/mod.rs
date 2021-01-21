use std::fmt::Debug;
use crate::ir::Opcode::Ret;

pub mod ir_gen;
mod tests;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Ret
}

#[derive(Debug, PartialEq)]
pub struct Quad {
    op: Opcode,
    op_type: String,
    src1: String,
    src2: String,
}

impl Quad {
    pub fn ret(data: Data) -> Self {
        Quad {
            op: Ret,
            op_type: data._type,
            src1: data.value,
            src2: "".to_string()
        }
    }
}

pub struct BasicBlock {
    name: String,
    quads: Vec<Quad>,
}

pub trait IRGen {
    fn generate(&self, cxt: &mut IRGenContext) -> Result<(), String>;
}

pub struct Data {
    _type: String,
    value: String,
}

pub struct IRGenContext {
    pub basic_blocks: Vec<BasicBlock>,
    data_stack: Vec<Data>,
    quads: Vec<Quad>,
}

impl IRGenContext {
    pub fn new() -> Self {
        IRGenContext {
            basic_blocks: vec![],
            data_stack: vec![],
            quads: vec![],
        }
    }

    pub fn push_data(&mut self, data: Data) {
        self.data_stack.push(data);
    }

    pub fn pop_data(&mut self) -> Option<Data> {
        self.data_stack.pop()
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn add_basic_blocks(&mut self, basic_block: BasicBlock) {
        self.basic_blocks.push(basic_block);
    }

    pub fn swap_and_get_quads(&mut self) -> Vec<Quad> {
        let mut quads = vec![];
        std::mem::swap(&mut quads, &mut self.quads);
        quads
    }
}
