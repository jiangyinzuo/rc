//! Semantic analysis includes:
//! - Symbol resolving
//! - Type check
//! - Flow control check
//!
pub mod file;

use crate::rcc::RccError;

/// Semantic analysis
trait SemAnalyse {
    fn analyse(&self, cxt: &mut AnalyseContext) -> Result<(), RccError>;
}

struct AnalyseContext {

}