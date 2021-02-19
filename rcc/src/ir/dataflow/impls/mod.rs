use crate::ir::cfg::CFG;

mod live_variable;

pub struct LiveVariableAnalysis<'cfg> {
    cfg: &'cfg CFG,
}