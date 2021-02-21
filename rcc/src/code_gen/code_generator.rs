use std::io::{BufWriter, Write};
use crate::ir::cfg::{CFGIR, CFG};
use crate::rcc::RccError;

pub struct CodeGen<'w, W: Write> {
    cfg_ir: CFGIR,
    output: &'w mut BufWriter<W>,
}

impl<'w, W: 'w + Write> CodeGen<'w, W> {
    pub fn new(cfg_ir: CFGIR, output: &'w mut BufWriter<W>) -> CodeGen<W> {
        CodeGen { cfg_ir, output }
    }

    pub fn run(&mut self) -> Result<(), RccError> {
        self.gen_read_only_local_str()?;
        self.gen_functions()?;
        Ok(())
    }

    fn gen_read_only_local_str(&mut self) -> Result<(), RccError> {
        writeln!(self.output, "\t.text")?;
        if !self.cfg_ir.ro_local_strs.is_empty() {
            writeln!(self.output, "\t.section\t.rodata")?;
        }
        for s in self.cfg_ir.ro_local_strs.iter() {
            writeln!(self.output, "{}:", s.0)?;
            writeln!(self.output, "\t.string \"{}\"", s.1)?;
        }
        Ok(())
    }

    fn gen_functions(&mut self) -> Result<(), RccError> {
        writeln!(self.output, "\t.text")?;
        for cfg in self.cfg_ir.cfgs.iter() {
            let mut func_gen = FuncCodeGen::new(cfg, self.output);
            func_gen.gen_function()?;
        }
        Ok(())
    }
}

struct FuncCodeGen<'w: 'codegen, 'codegen, W: Write> {
    cfg: &'codegen CFG,
    output: &'w mut BufWriter<W>,
    frame_size: u32,
}

impl<'w: 'codegen, 'codegen, W: Write> FuncCodeGen<'w, 'codegen, W> {
    fn new(
        cfg: &'codegen CFG,
        output: &'w mut BufWriter<W>,
    ) -> FuncCodeGen<'w, 'codegen, W> {
        let frame_size = 0;
        FuncCodeGen { cfg, output, frame_size }
    }

    fn gen_function(&mut self) -> Result<(), RccError> {
        if self.cfg.func_is_global {
            writeln!(self.output, "\t.globl  {}", self.cfg.func_name)?;
        }
        writeln!(self.output, "{}:", self.cfg.func_name)?;
        self.gen_function_entry()?;
        writeln!(self.output, "\tret")?;
        Ok(())
    }

    fn gen_function_entry(&mut self) -> Result<(), RccError> {
        Ok(())
    }
}
