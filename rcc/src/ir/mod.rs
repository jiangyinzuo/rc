use crate::analyser::sym_resolver::{TypeInfo, VarInfo, VarKind};
use crate::ast::expr::BinOperator;
use crate::ast::types::TypeLitNum;
use crate::rcc::RccError;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;

pub mod cfg;
mod dataflow;
pub mod ir_build;
mod linear_ir;
pub(crate) mod tests;

#[derive(Debug, PartialEq)]
pub enum Jump {
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
    FnLabel(String),
    Unit,
    Never,
    FnRetPlace,
}

impl Operand {
    pub fn is_unit_or_never(&self) -> bool {
        matches!(self, Self::Unit | Self::Never)
    }

    pub fn eq_or_is_never(&self, other: Operand) -> bool {
        self == &other || self == &Self::Never
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    pub label: String,
    pub kind: VarKind,
    pub ir_type: IRType,
}

impl Place {
    pub fn label(ident: &str, scope_id: u64) -> String {
        format!("{}_{}", ident, scope_id)
    }

    pub fn new(label: String, kind: VarKind, ir_type: IRType) -> Place {
        Place {
            label,
            kind,
            ir_type,
        }
    }

    pub fn variable(ident: &str, scope_id: u64, var_kind: VarKind, ir_type: IRType) -> Place {
        Place::new(Self::label(ident, scope_id), var_kind, ir_type)
    }

    pub fn local(label: String, ir_type: IRType) -> Place {
        Place {
            label,
            kind: VarKind::Local,
            ir_type,
        }
    }

    pub fn local_mut(label: String, ir_type: IRType) -> Place {
        Place {
            label,
            kind: VarKind::LocalMut,
            ir_type,
        }
    }

    pub fn lit_const(label: String, ir_type: IRType) -> Place {
        Place {
            label,
            kind: VarKind::LitConst,
            ir_type,
        }
    }

    pub fn is_temp(&self) -> bool {
        self.label.starts_with("$")
    }
}

pub struct Func {
    pub name: String,
    insts: VecDeque<IRInst>,
    pub is_global: bool,
}

impl Func {
    pub fn new(name: String, is_global: bool) -> Func {
        Func {
            name,
            insts: VecDeque::new(),
            is_global,
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
    /// zero sized type
    Unit,
    Never,
    /// address
    Addr,
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
            TypeInfo::Unit => IRType::Unit,
            TypeInfo::Never => IRType::Never,
            TypeInfo::Ptr { .. } => IRType::Addr,
            t => return Err(RccError::Parse(format!("invalid type {:?}", t))),
        };
        Ok(ir_type)
    }

    pub fn from_var_info(var_info: &VarInfo) -> Result<IRType, RccError> {
        let t = var_info.type_info.borrow();
        let tp = t.deref();
        Self::from_type_info(tp)
    }
}

/// Immediate Presentation's Instructions
#[derive(Debug, PartialEq)]
pub enum IRInst {
    BinOp {
        op: BinOperator,
        dest: Place,
        src1: Operand,
        src2: Operand,
    },

    Jump {
        label: usize,
    },

    JumpIfCond {
        cond: Jump,
        src1: Operand,
        src2: Operand,
        label: usize,
    },

    JumpIf {
        cond: Operand,
        label: usize,
    },

    JumpIfNot {
        cond: Operand,
        label: usize,
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
        callee: Operand,
        args: Vec<Operand>,
    },

    Ret(Operand),
}

impl IRInst {
    pub fn bin_op(op: BinOperator, dest: Place, src1: Operand, src2: Operand) -> IRInst {
        IRInst::BinOp {
            op,
            dest,
            src1,
            src2,
        }
    }

    pub fn load_data(dest: Place, src: Operand) -> IRInst {
        IRInst::LoadData { dest, src }
    }

    pub fn jump(label: usize) -> IRInst {
        IRInst::Jump { label }
    }

    pub fn jump_if(cond: Operand, label: usize) -> IRInst {
        IRInst::JumpIf { cond, label }
    }

    pub fn jump_if_not(cond: Operand, label: usize) -> IRInst {
        IRInst::JumpIfNot { cond, label }
    }

    pub fn jump_if_cond(cond: Jump, src1: Operand, src2: Operand, label: usize) -> IRInst {
        IRInst::JumpIfCond {
            cond,
            src1,
            src2,
            label,
        }
    }

    pub fn call(callee: Operand, args: Vec<Operand>) -> IRInst {
        IRInst::Call { callee, args }
    }

    pub fn set_jump_label(&mut self, new_label: usize) {
        match self {
            Self::Jump { label } => *label = new_label,
            Self::JumpIfNot { cond, label } => *label = new_label,
            Self::JumpIf { cond, label } => *label = new_label,
            Self::JumpIfCond {
                cond,
                src1,
                src2,
                label,
            } => *label = new_label,
            _ => unreachable!(),
        }
    }

    pub fn jump_label(&self) -> usize {
        *match self {
            Self::Jump { label } => label,
            Self::JumpIfNot { cond, label } => label,
            Self::JumpIf { cond, label } => label,
            Self::JumpIfCond {
                cond,
                src1,
                src2,
                label,
            } => label,
            ir => unreachable!("{:?}", ir),
        }
    }
}

pub enum StrKind {
    Lit,
    Const,
}
