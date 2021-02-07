pub mod ir_build;

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum BinOp {
    /// Shifts
    SLL,
    SRL,
    SRA,

    /// Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    /// Logical
    Xor,
    Or,
    And,

    /// Compare
    LT,
    GT,
    LE,
    GE,
}

pub enum Jump {
    J,
    JEq,
    JNe,
    JLt,
    JGe,
}

pub enum Operand {
    Imm(IRType),
    Place(Place),
}

pub struct Place {}

pub struct Func {
    name: String,
    insts: Vec<IRInst>,
}

pub enum IRType {
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

/// Immediate Presentation's Instructions
pub enum IRInst {
    BinOp {
        op: BinOp,
        _type: IRType,
        dest: Place,
        src1: Operand,
        src2: Operand,
    },

    Jump {
        label: String,
    },

    JumpIf {
        cond: Jump,
        src1: Operand,
        src2: Operand,
        label: String,
    },

    LoadData {
        dest: Place,
        src: Operand,
    },

    LoadAddr {
        dest: Place,
        symbol: Operand,
    },

    Call {
        func_name: String,
        args: Vec<Operand>,
    },

    Ret(Operand),
}

pub struct IR {
    funcs: Vec<Func>
}

impl IR {
    pub fn new () -> IR {
        IR {funcs: vec![]}
    }
}
