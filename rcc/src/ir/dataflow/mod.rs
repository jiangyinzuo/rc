use crate::ir::cfg::CFG;
use bit_vector::BitVector;

mod live_variable;
mod tests;
pub mod reaching_definitions;

trait AnalysisDomain {
    fn bottom_value(cfg: &CFG) -> Self;
}

impl AnalysisDomain for BitVector {
    fn bottom_value(cfg: &CFG) -> Self {
        BitVector::new(cfg.local_variables.len())
    }
}
