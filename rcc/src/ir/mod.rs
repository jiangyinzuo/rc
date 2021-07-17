use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;

use crate::analyser::sym_resolver::{TypeInfo, VarInfo, VarKind};
use crate::ast::expr::BinOperator;
use crate::ast::types::TypeLitNum;
use crate::ir::var_name::{is_temp_var, local_var};
use crate::rcc::RccError;

pub mod cfg;
mod dataflow;
pub mod ir_build;
mod linear_ir;
pub(crate) mod tests;
pub mod var_name;

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
    FnRetPlace(IRType),
}

impl Operand {
    pub fn byte_size(&self, addr_size: u32) -> u32 {
        match self {
            Self::Unit | Self::Never => 0,
            Self::Bool(_) | Self::Char(_)| Self::I8(_) | Self::U8(_) => 1,
            Self::I32(_) | Self::U32(_) => 4,
            Self::I64(_) | Self::U64(_) => 8,
            Self::Place(p) => p.ir_type.byte_size(addr_size),
            Self::FnRetPlace(ir_type) => ir_type.byte_size(addr_size),
            _ => unimplemented!("{:?}", self),
        }
    }

    pub fn is_imm(&self) -> bool {
        matches!(self, Self::Bool(_) | Self::Char(_) |
         Self::F32(_) | Self::F64(_) |
         Self::I8(_) | Self::U8(_) |
         Self::I16(_) | Self::U16(_) |
                       Self::I32(_) | Self::U32(_) |
                       Self::I64(_) | Self::U64(_) |
                       Self::I128(_) | Self::U128(_) |
                       Self::Isize(_) | Self::Usize(_))
    }
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
    pub fn new(label: String, kind: VarKind, ir_type: IRType) -> Place {
        Place {
            label,
            kind,
            ir_type,
        }
    }

    pub fn variable(ident: &str, scope_id: u64, var_kind: VarKind, ir_type: IRType) -> Place {
        Place::new(local_var(ident, scope_id), var_kind, ir_type)
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
        is_temp_var(&self.label)
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
    pub fn byte_size(&self, addr_size: u32) -> u32 {
        match self {
            IRType::I8 | IRType::U8 | IRType::Char | IRType::Bool => 1,
            IRType::I16 | IRType::U16 => 2,
            IRType::I32 | IRType::U32 | IRType::F32 => 4,
            IRType::I64 | IRType::U64 | IRType::F64 => 8,
            IRType::I128 | IRType::U128 => 16,
            IRType::Isize | IRType::Usize | IRType::Addr => {
                debug_assert!(addr_size % 8 == 0);
                addr_size / 8
            }
            IRType::Unit | IRType::Never => 0,
        }
    }

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

    /// dest = src1 op src2
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

    /// dest = src
    LoadData {
        dest: Place,
        src: Operand,
    },

    /// dest = *symbol
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
        debug_assert!(!src1.is_imm() || !src2.is_imm());
        if src2.is_imm() {
            IRInst::BinOp {
                op,
                dest,
                src1,
                src2,
            }
        } else {
            IRInst::BinOp {
                op,
                dest,
                src2,
                src1,
            }
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

/// Constant fold optimization.
/// a = 2 * 3 -> a = 6
/// TODO other primitive type
pub fn bin_op_may_constant_fold(
    op: &BinOperator,
    src1: &Operand,
    src2: &Operand,
) -> Result<Option<Operand>, RccError> {
    macro_rules! try_fold_int {
        ($i:path, $l:ident, $r:ident) => {
            match op {
                BinOperator::Plus => Some($i(match $l.checked_add(*$r) {
                    Some(res) => res,
                    None => return Err("add overflow".into()),
                })),
                BinOperator::Minus => Some($i(match $l.checked_sub(*$r) {
                    Some(res) => res,
                    None => return Err("sub overflow".into()),
                })),
                BinOperator::Star => Some($i(match $l.checked_mul(*$r) {
                    Some(res) => res,
                    None => return Err("mul overflow".into()),
                })),
                BinOperator::Slash => Some($i(match $l.checked_div(*$r) {
                    Some(res) => res,
                    None => return Err("div overflow".into()),
                })),
                BinOperator::Lt => Some(Operand::Bool($l < $r)),
                BinOperator::Le => Some(Operand::Bool($l <= $r)),
                BinOperator::Gt => Some(Operand::Bool($l > $r)),
                BinOperator::Ge => Some(Operand::Bool($l >= $r)),
                BinOperator::Ne => Some(Operand::Bool($l != $r)),
                BinOperator::EqEq => Some(Operand::Bool($l == $r)),
                BinOperator::Shl => Some($i(match $l.checked_shl(*$r as u32) {
                    Some(res) => res,
                    None => return Err("shl overflow".into()),
                })),
                BinOperator::Shr => Some($i(match $l.checked_shr(*$r as u32) {
                    Some(res) => res,
                    None => return Err("shr overflow".into()),
                })),
                BinOperator::And => Some($i($l & $r)),
                BinOperator::Or => Some($i($l | $r)),
                BinOperator::Caret => Some($i($l ^ $r)),
                BinOperator::Percent => Some($i(match $l.checked_rem(*$r) {
                    Some(res) => res,
                    None => return Err("rem overflow".into()),
                })),
                _ => None,
            }
        };
    }
    Ok(match (src1, src2) {
        (Operand::I32(l), Operand::I32(r)) => try_fold_int!(Operand::I32, l, r),
        (Operand::I64(l), Operand::I64(r)) => try_fold_int!(Operand::I64, l, r),
        (Operand::I128(l), Operand::I128(r)) => try_fold_int!(Operand::I128, l, r),
        _ => None,
    })
}
