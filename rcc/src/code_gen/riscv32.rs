//! b(byte): 8bit
//! h(half word): 16bit
//! w(word): 32bit
use crate::analyser::sym_resolver::VarKind;
use crate::ast::expr::BinOperator;
use crate::code_gen::{create_allocator, Allocator};
use crate::ir::cfg::{CFG, CFGIR};
use crate::ir::var_name::{branch_name, FP, RA};
use crate::ir::{IRInst, IRType, Jump, Operand, Place};
use crate::rcc::{OptimizeLevel, RccError};
use std::io::{BufWriter, Write};

const RISCV32_ADDR_SIZE: u32 = 32;
pub struct Riscv32CodeGen<'w, W: Write> {
    cfg_ir: CFGIR,
    output: &'w mut BufWriter<W>,
    opt_level: OptimizeLevel,
}

impl<'w, W: 'w + Write> Riscv32CodeGen<'w, W> {
    pub fn new(
        cfg_ir: CFGIR,
        output: &'w mut BufWriter<W>,
        opt_level: OptimizeLevel,
    ) -> Riscv32CodeGen<W> {
        Riscv32CodeGen {
            cfg_ir,
            output,
            opt_level,
        }
    }

    pub fn run(&mut self) -> Result<(), RccError> {
        self.gen_read_only_local_str()?;
        self.gen_functions()?;
        Ok(())
    }

    fn gen_read_only_local_str(&mut self) -> Result<(), RccError> {
        if !self.cfg_ir.ro_local_strs.is_empty() {
            writeln!(self.output, "\t.text")?;
            writeln!(self.output, "\t.section\t.rodata")?;
            for s in self.cfg_ir.ro_local_strs.iter() {
                writeln!(self.output, "{}:", s.0)?;
                writeln!(self.output, "\t.string \"{}\"", s.1)?;
            }
        }
        Ok(())
    }

    fn gen_functions(&mut self) -> Result<(), RccError> {
        writeln!(self.output, "\t.text")?;
        for cfg in self.cfg_ir.cfgs.iter() {
            let mut func_gen = FuncCodeGen::new(cfg, self.output, self.opt_level);
            func_gen.gen_function()?;
        }
        Ok(())
    }
}

struct FuncCodeGen<'w: 'codegen, 'codegen, W: Write> {
    cfg: &'codegen CFG,
    output: &'w mut BufWriter<W>,
    allocator: Box<dyn Allocator + 'codegen>,
    frame_size: u32,
}

impl<'w: 'codegen, 'codegen, W: Write> FuncCodeGen<'w, 'codegen, W> {
    fn new(
        cfg: &'codegen CFG,
        output: &'w mut BufWriter<W>,
        opt_level: OptimizeLevel,
    ) -> FuncCodeGen<'w, 'codegen, W> {
        let allocator = create_allocator(opt_level, cfg, RISCV32_ADDR_SIZE);
        let frame_size = allocator.get_frame_size();
        FuncCodeGen {
            cfg,
            output,
            allocator,
            frame_size,
        }
    }

    /// # Calling convention and stack frame of RC
    ///
    /// [calling convention of minidecaf]: https://decaf-lang.github.io/minidecaf-tutorial/docs/lab9/calling.html
    /// [RISC-V ELF psABI specification]: https://github.com/riscv/riscv-elf-psabi-doc/blob/master/riscv-elf.md
    /// [Volume I: RISC-V User-Level ISA V2.1draft]: https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf
    /// [RICS-V ISA Specifications]: https://riscv.org/technical/specifications/
    ///
    /// ## Example
    ///
    /// ```
    /// fn foo(arg0: i32, arg1: i32, arg2: i32, arg3: i32,
    ///        arg4: i32, arg5: i32, arg6: i32, arg7: i32,
    ///        arg8: i32, arg9: i32) {
    /// }
    /// a0: arg0, a1: arg1, ...
    ///
    ///
    /// High Address
    ///
    /// |  ...   |
    /// +--------+     |
    /// |  arg9  |     |-- stack frame of foo's caller
    /// +--------+     |
    /// |  arg8  |     |
    /// +--------+ <---- fp(s0)
    /// |   ra   |     |
    /// +--------+     |
    /// | old fp |     |-- stack frame of function foo
    /// +--------+     |
    /// |  arg0  |     |
    /// |  arg1  |     |
    /// |  ...   |     |
    /// |  arg7  |     |
    /// +--------+     |
    /// | callee |     |
    /// | saved  |     |
    /// |(s1-s11)|     |
    /// +--------+     |
    /// | locals |     |
    /// +--------+ <---- sp
    ///
    /// Low Address
    /// ```
    fn gen_function(&mut self) -> Result<(), RccError> {
        if self.cfg.func_is_global {
            writeln!(self.output, "\t.globl  {}", self.cfg.func_name)?;
        }
        writeln!(self.output, "{}:", self.cfg.func_name)?;
        if !self.cfg.basic_blocks.is_empty() {
            self.gen_function_entry()?;
            self.gen_save_args()?;
            self.gen_instructions()?;
            self.gen_exit_function()?;
        }
        writeln!(self.output, "\tret")?;
        Ok(())
    }

    fn gen_function_entry(&mut self) -> Result<(), RccError> {
        debug_assert!(self.frame_size >= 8);
        // set sp
        writeln!(self.output, "\taddi\tsp,sp,-{}", self.frame_size)?;
        if !self.cfg.is_leaf {
            // save ra
            let offset = self.allocator.get_fp_offset(RA, &IRType::Addr);
            debug_assert_eq!(4, offset);
            writeln!(self.output, "\tsw\tra,{}(sp)", self.frame_size - 4)?;
        }
        // save old fp(s0)
        let offset = self.allocator.get_fp_offset(FP, &IRType::Addr);
        writeln!(self.output, "\tsw\ts0,{}(sp)", self.frame_size - offset)?;
        // set fp
        writeln!(self.output, "\taddi\ts0,sp,{}", self.frame_size)?;
        Ok(())
    }

    fn gen_exit_function(&mut self) -> Result<(), RccError> {
        if !self.cfg.is_leaf {
            // restore ra
            let offset = self.allocator.get_fp_offset(RA, &IRType::Addr);
            debug_assert_eq!(4, offset);
            writeln!(self.output, "\tlw\tra,{}(sp)", self.frame_size - offset)?;
        }
        // restore old fp
        let offset = self.allocator.get_fp_offset(FP, &IRType::Addr);
        writeln!(self.output, "\tlw\ts0,{}(sp)", self.frame_size - offset)?;
        // restore sp
        writeln!(self.output, "\taddi\tsp,sp,{}", self.frame_size)?;
        Ok(())
    }

    fn gen_save_args(&mut self) -> Result<(), RccError> {
        for i in 0..self.cfg.fn_args.len().min(8) {
            let arg_name = self.cfg.get_name_of_fn_arg(i).unwrap();
            let (_, ir_type) = self.cfg.local_infos.get(&arg_name).unwrap();
            let offset = self.allocator.get_fp_offset(&arg_name, ir_type);
            writeln!(self.output, "\tsw\ta{},-{}(s0)", i, offset)?;
        }
        Ok(())
    }

    fn gen_instructions(&mut self) -> Result<(), RccError> {
        for bb in self.cfg.basic_blocks.iter() {
            if !bb.predecessors.is_empty() {
                writeln!(self.output, "{}:", branch_name(self.cfg.func_scope_id, bb.id))?;
            }
            for inst in bb.instructions.iter() {
                self.gen_instruction(inst)?;
            }
        }
        Ok(())
    }

    fn gen_instruction(&mut self, inst: &IRInst) -> Result<(), RccError> {
        match inst {
            IRInst::Ret(o) => self.load_data("a0", o)?,
            IRInst::LoadData { dest, src } => match dest.kind {
                VarKind::Local | VarKind::LocalMut => {
                    let offset = self.allocator.get_fp_offset(&dest.label, &dest.ir_type);
                    self.load_data("a5", src)?;
                    let size = src.byte_size(RISCV32_ADDR_SIZE);
                    self.store_data(size, "a5", -(offset as i32), "s0")?;
                }
                _ => unimplemented!(),
            },
            IRInst::BinOp {
                op,
                dest,
                src1,
                src2,
            } => {
                debug_assert!(!src1.is_imm());
                if src2.is_imm() {
                    self.load_data("a5", src1)?;
                    self.bin_op_imm(op, dest, "a5", src2)?;
                } else {
                    self.load_data("a4", src1)?;
                    self.load_data("a5", src2)?;
                    self.bin_op(op, dest, "a4", "a5")?;
                }
            }
            IRInst::Call { callee, args } => match callee {
                Operand::FnLabel(fn_name) => {
                    self.pass_fn_args(args)?;
                    writeln!(self.output, "\tcall\t{}", fn_name)?;
                }
                _ => unreachable!(),
            },
            IRInst::Jump { label } => {
                writeln!(self.output, "\tj\t{}", branch_name(self.cfg.func_scope_id, *label))?;
            }
            IRInst::JumpIfCond {
                cond,
                src1,
                src2,
                label,
            } => {
                self.load_data("a4", src1)?;
                self.load_data("a5", src2)?;
                let inst = match cond {
                    Jump::JEq => "beq",
                    Jump::JGe => "ble",
                    Jump::JLt => "bgt",
                    Jump::JNe => "beq",
                };
                writeln!(self.output, "\t{}\ta5,a4,{}", inst, branch_name(self.cfg.func_scope_id, *label))?;
            }
            IRInst::JumpIfNot { cond, label } => {
                self.load_data("a5", cond)?;
                // writeln!(self.output, "\t")?;
                todo!()
            }
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    fn pass_fn_args(&mut self, args: &[Operand]) -> Result<(), RccError> {
        for (i, arg) in args.iter().enumerate() {
            // pass by registers
            if i <= 7 {
                self.load_data(&format!("a{}", i), arg)?;
            }
        }
        Ok(())
    }

    fn load_data(&mut self, reg_name: &str, operand: &Operand) -> Result<(), RccError> {
        let asm_operand = AsmOperand::from_operand(operand, &mut *self.allocator);
        let size = operand.byte_size(RISCV32_ADDR_SIZE);
        match asm_operand {
            AsmOperand::Imm(s) => {
                writeln!(self.output, "\tli\t{},{}", reg_name, s)?;
            }
            AsmOperand::FpOffset(offset) => {
                let inst = match size {
                    4 => "lw",
                    _ => todo!(),
                };
                writeln!(self.output, "\t{}\t{},-{}(s0)", inst, reg_name, offset)?;
            }
            AsmOperand::Never | AsmOperand::Unit => {}
            AsmOperand::FnRet(_ir_type) => match size {
                4 => {
                    if reg_name != "a0" {
                        writeln!(self.output, "\tmv\t{},a0", reg_name)?;
                    }
                }
                _ => todo!(),
            },
            _ => unimplemented!("{:?}", asm_operand),
        }
        Ok(())
    }

    /// sb(store byte), sh(store half-word), sw(store word)
    fn store_data(
        &mut self,
        src_byte_size: u32,
        src_reg_name: &str,
        offset: i32,
        tar_reg_name: &str,
    ) -> Result<(), RccError> {
        let inst = match src_byte_size {
            1 => "sb",
            2 => "sh",
            4 => "sw",
            _ => todo!(),
        };
        writeln!(
            self.output,
            "\t{}\t{},{}({})",
            inst, src_reg_name, offset, tar_reg_name
        )?;
        Ok(())
    }

    fn bin_op(
        &mut self,
        op: &BinOperator,
        dest: &Place,
        reg_src1: &str,
        reg_src2: &str,
    ) -> Result<(), RccError> {
        match dest.kind {
            VarKind::LocalMut | VarKind::Local => {
                let offset = self.allocator.get_fp_offset(&dest.label, &dest.ir_type);
                let inst = match op {
                    BinOperator::Plus => "add",
                    BinOperator::Star => "mul",
                    BinOperator::Minus => "sub",
                    BinOperator::Slash => "div",
                    BinOperator::Percent => match dest.ir_type {
                        IRType::I8 | IRType::I16 | IRType::I32 => "rem",
                        IRType::U8 | IRType::U16 | IRType::U32 => "remu",
                        _ => unimplemented!(),
                    },
                    _ => todo!(),
                };
                writeln!(self.output, "\t{}\ta5,{},{}", inst, reg_src1, reg_src2)?;
                self.store_data(
                    dest.ir_type.byte_size(RISCV32_ADDR_SIZE),
                    "a5",
                    -(offset as i32),
                    "s0",
                )?;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn bin_op_imm(
        &mut self,
        op: &BinOperator,
        dest: &Place,
        reg_src1: &str,
        src2: &Operand,
    ) -> Result<(), RccError> {
        let asm_src2 = AsmOperand::from_operand(src2, &mut *self.allocator);
        match asm_src2 {
            AsmOperand::Imm(s) => match dest.kind {
                VarKind::LocalMut | VarKind::Local => {
                    let offset = self.allocator.get_fp_offset(&dest.label, &dest.ir_type);
                    match op {
                        BinOperator::Plus => {
                            writeln!(self.output, "\taddi\ta5,{},{}", reg_src1, s)?;
                            self.store_data(
                                dest.ir_type.byte_size(RISCV32_ADDR_SIZE),
                                "a5",
                                -(offset as i32),
                                "s0",
                            )?;
                        }
                        BinOperator::Minus => {
                            writeln!(self.output, "\taddi\ta5,{},-{}", reg_src1, s)?;
                            self.store_data(
                                dest.ir_type.byte_size(RISCV32_ADDR_SIZE),
                                "a5",
                                -(offset as i32),
                                "s0",
                            )?;
                        }
                        _ => {
                            self.load_data("a4", &src2)?;
                            self.bin_op(op, dest, reg_src1, "a4")?;
                        }
                    }
                }
                _ => unimplemented!(),
            },
            _ => todo!(),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AsmOperand {
    Imm(String),
    Imm64(String, String),
    Imm128(String, String, String, String),
    Reg(String),
    FpOffset(u32),
    Never,
    Unit,
    FnRet(IRType),
}

impl AsmOperand {
    pub fn from_operand(operand: &Operand, allocator: &mut dyn Allocator) -> AsmOperand {
        match operand {
            Operand::Char(c) => Self::Imm((*c as u8).to_string()),
            Operand::I8(i) => Self::Imm(i.to_string()),
            Operand::I16(i) => Self::Imm(i.to_string()),
            Operand::I32(i) => Self::Imm(i.to_string()),
            Operand::U8(i) => Self::Imm(i.to_string()),
            Operand::U16(i) => Self::Imm(i.to_string()),
            Operand::U32(i) => Self::Imm(i.to_string()),
            Operand::Place(p) => {
                match p.kind {
                    VarKind::Local | VarKind::LocalMut => {
                        Self::FpOffset(allocator.get_fp_offset(&p.label, &p.ir_type))
                    }
                    // todo
                    _ => Self::Unit,
                }
            }
            Operand::Unit => Self::Unit,
            Operand::Never => Self::Never,
            Operand::FnRetPlace(ir_type) => Self::FnRet(ir_type.clone()),
            _ => unimplemented!("{:?}", operand),
        }
    }
}
