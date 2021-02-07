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
    Imm,
    Place(Place),
}

pub struct Place {}

pub struct Func {
    name: Label,
}

pub type Label = String;

pub enum IR {
    BinOp {
        op: BinOp,
        dest: Place,
        src1: Operand,
        src2: Operand,
    },

    Jump {
        label: Label,
    },

    JumpIf {
        cond: Jump,
        src1: Operand,
        src2: Operand,
        label: Label,
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
        func: Func,
        args: Vec<Operand>,
    },
    Ret(Operand),
}
