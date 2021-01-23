use crate::analyser::{SemAnalyse, AnalyseContext};
use crate::ast::file::File;
use crate::rcc::RccError;
use crate::ast::NamedASTNode;

impl SemAnalyse for File {
    fn analyse(&self, cxt: &mut AnalyseContext) -> Result<(), RccError> {
        for item in &self.items {
            item.ident_name();
        }
        Ok(())
    }
}