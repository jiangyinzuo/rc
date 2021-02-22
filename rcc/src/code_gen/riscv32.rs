//! b(byte): 8bit
//! h(half word): 16bit
//! w(word): 32bit
use crate::code_gen::{create_allocator, Allocator};
use crate::ir::cfg::{CFG, CFGIR};
use crate::ir::var_name::{FP, RA};
use crate::ir::{IRType, IRInst, Operand};
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
            let offset = self.allocator.get_var_offset(RA, &IRType::Addr);
            debug_assert_eq!(4, offset);
            writeln!(self.output, "\tsw\tra,{}(sp)", self.frame_size - 4)?;
        }
        // save old fp(s0)
        let offset = self.allocator.get_var_offset(FP, &IRType::Addr);
        writeln!(self.output, "\tsw\ts0,{}(sp)", self.frame_size - offset)?;
        // set fp
        writeln!(self.output, "\taddi\ts0,sp,{}", self.frame_size)?;
        Ok(())
    }

    fn gen_exit_function(&mut self) -> Result<(), RccError> {
        if !self.cfg.is_leaf {
            // restore ra
            let offset = self.allocator.get_var_offset(RA, &IRType::Addr);
            debug_assert_eq!(4, offset);
            writeln!(self.output, "\tlw\tra,{}(sp)", self.frame_size - offset)?;
        }
        // restore old fp
        let offset = self.allocator.get_var_offset(FP, &IRType::Addr);
        writeln!(self.output, "\tlw\ts0,{}(sp)", self.frame_size - offset)?;
        // restore sp
        writeln!(self.output, "\taddi\tsp,sp,{}", self.frame_size)?;
        Ok(())
    }

    fn gen_save_args(&mut self) -> Result<(), RccError> {
        for i in 0..self.cfg.fn_args.len().min(8) {
            let arg_name = self.cfg.get_name_of_fn_arg(i).unwrap();
            let (_, ir_type) = self.cfg.local_infos.get(&arg_name).unwrap();
            let offset = self.allocator.get_var_offset(&arg_name, ir_type);
            writeln!(self.output, "\tsw\ta{},-{}(s0)", i, offset)?;
        }
        Ok(())
    }

    fn gen_instructions(&mut self) -> Result<(), RccError> {
        for inst in self.cfg.iter_inst() {
            self.gen_instruction(inst)?;
        }
        Ok(())
    }

    fn gen_instruction(&mut self, inst: &IRInst)-> Result<(), RccError> {
        match inst {
            IRInst::Ret(o) => {
                match o {
                    Operand::I32(i) => {}
                    _ => {}
                }
            }
            _ => {
                // TODO
            }
        }
        Ok(())
    }
}