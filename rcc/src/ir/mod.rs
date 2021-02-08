pub mod ir_build;

use std::collections::HashMap;
use std::fmt::Debug;
use crate::ir::Place::Label;

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

pub enum Place {
    Label(String),
}

pub struct Func {
    name: String,
    insts: Vec<IRInst>,
}

impl Func {
    pub fn new() -> Func {
        Func {
            name: "".to_string(),
            insts: vec![]
        }
    }
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

pub enum StrKind {
    Lit,
    Const,
}

pub struct IR {
    funcs: Vec<Func>,
    strs: HashMap<String, String>,
}

impl IR {
    pub fn new() -> IR {
        IR {
            funcs: vec![],
            strs: HashMap::new(),
        }
    }

    pub fn add_lit_str(&mut self, s: String) -> Operand {
        let label = format!(".LC{}", self.strs.len());
        self.strs.insert(label.clone(), s);
        Operand::Place(Label(label))
    }

    pub fn add_func(&mut self, func: Func) {
        self.funcs.push(func);
    }
}
