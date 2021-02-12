pub mod ir_build;
mod tests;

use crate::analyser::sym_resolver::TypeInfo;
use crate::ast::expr::BinOperator;
use crate::ast::types::TypeLitNum;
use crate::ir::Place::Label;
use crate::rcc::RccError;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Jump {
    J,
    JEq,
    JNe,
    JLt,
    JGe,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
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
    Place(Place),
    Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Place {
    Label(String),
    Var(String),
}

impl Place {
    pub fn var(ident: String) -> Place {
        Place::Var(ident)
    }
}

pub struct Func {
    name: String,
    insts: Vec<IRInst>,
}

impl Func {
    pub fn new() -> Func {
        Func {
            name: "".to_string(),
            insts: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IRType {
    F32,
    F64,
    Bool,
    Char,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl IRType {
    pub fn from_type_info(type_info: &TypeInfo) -> Result<IRType, RccError> {
        let ir_type = match type_info {
            TypeInfo::LitNum(num) => match num {
                TypeLitNum::F32 => IRType::F32,
                TypeLitNum::F | TypeLitNum::F64 => IRType::F64,
                TypeLitNum::I8 => IRType::I8,
                TypeLitNum::I16 => IRType::I16,
                TypeLitNum::I | TypeLitNum::I32 => IRType::I32,
                TypeLitNum::I64 => IRType::I64,
                TypeLitNum::I128 => IRType::I128,
                TypeLitNum::Isize => IRType::Isize,
                TypeLitNum::U8 => IRType::U8,
                TypeLitNum::U16 => IRType::U16,
                TypeLitNum::U32 => IRType::U32,
                TypeLitNum::U64 => IRType::U64,
                TypeLitNum::U128 => IRType::U128,
                TypeLitNum::Usize => IRType::Usize,
            },
            TypeInfo::Bool => IRType::Bool,
            TypeInfo::Char => IRType::Char,
            t => return Err(RccError::Parse(format!("invalid type {:?}", t).into())),
        };
        Ok(ir_type)
    }
}

/// Immediate Presentation's Instructions
#[derive(Debug, PartialEq)]
pub enum IRInst {
    BinOp {
        op: BinOperator,
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

impl IRInst {
    pub fn bin_op(
        op: BinOperator,
        _type: IRType,
        dest: Place,
        src1: Operand,
        src2: Operand,
    ) -> IRInst {
        IRInst::BinOp {
            op,
            _type,
            dest,
            src1,
            src2,
        }
    }

    pub fn load_data(dest: Place, src: Operand) -> IRInst {
        IRInst::LoadData { dest, src }
    }
}

pub enum StrKind {
    Lit,
    Const,
}

pub struct IR {
    funcs: Vec<Func>,
    strs: HashMap<String, String>,
    instructions: Vec<IRInst>,
}

impl IR {
    pub fn new() -> IR {
        IR {
            funcs: vec![],
            strs: HashMap::new(),
            instructions: vec![],
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

    pub fn add_instructions(&mut self, ir_inst: IRInst) {
        self.instructions.push(ir_inst);
    }
}
