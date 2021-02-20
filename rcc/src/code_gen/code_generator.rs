use crate::ir::IR;
use std::io::{BufWriter, Write};
use crate::rcc::RccError;

pub struct CodeGenerator<'w, W: Write> {
    ir: IR,
    output: &'w mut BufWriter<W>
}

impl<'w, W: Write> CodeGenerator<'w, W> {
    pub fn new(ir: IR, output: &'w mut BufWriter<W>) -> CodeGenerator<W> {
        CodeGenerator { ir, output }
    }

    pub fn run(&mut self,) -> Result<(), RccError> {
        self.write_read_only_local_str()?;
        Ok(())
    }

    fn write_read_only_local_str(&mut self) -> Result<(), RccError> {
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
}
