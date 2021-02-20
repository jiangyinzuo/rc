use crate::ir::{Func, IR};
use crate::rcc::RccError;
use std::io::{BufWriter, Write};

pub struct CodeGenerator<'w, W: Write> {
    ir: IR,
    output: &'w mut BufWriter<W>,
}

impl<'w, W: Write> CodeGenerator<'w, W> {
    pub fn new(ir: IR, output: &'w mut BufWriter<W>) -> CodeGenerator<W> {
        CodeGenerator { ir, output }
    }

    pub fn run(&mut self) -> Result<(), RccError> {
        self.gen_read_only_local_str()?;
        self.gen_functions()?;
        Ok(())
    }

    fn gen_read_only_local_str(&mut self) -> Result<(), RccError> {
        writeln!(self.output, "\t.text")?;
        if !self.ir.ro_local_strs.is_empty() {
            writeln!(self.output, "\t.section\t.rodata")?;
        }
        for s in self.ir.ro_local_strs.iter() {
            writeln!(self.output, "{}:", s.0)?;
            writeln!(self.output, "\t.string \"{}\"", s.1)?;
        }
        Ok(())
    }

    fn gen_functions(&mut self) -> Result<(), RccError> {
        writeln!(self.output, "\t.text")?;
        for func in self.ir.funcs.iter() {
            Self::gen_function(self.output, func)?;
        }
        Ok(())
    }

    fn gen_function(output: &mut BufWriter<W>, func: &Func) -> Result<(), RccError> {
        if func.is_global {
            writeln!(output, "\t.globl  {}", func.name)?;
        }
        writeln!(output, "{}:", func.name)?;
        Ok(())
    }
}
