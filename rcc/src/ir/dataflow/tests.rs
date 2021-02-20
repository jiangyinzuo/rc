use crate::ir::cfg::CFG;
use crate::ir::tests::ir_build;
use crate::rcc::RccError;
use crate::ir::dataflow::live_variable::LiveVariableAnalysis;

fn get_cfg(input: &str) -> Result<CFG, RccError> {
    let mut ir = ir_build(input)?;
    debug_assert_eq!(1, ir.funcs.len());
    let cfg = CFG::new(ir.funcs.pop().unwrap());
    Ok(cfg)
}

#[test]
fn live_variable_test() {
    let cfg = get_cfg(r#"
        fn fooo() {
            let mut a = 3;
            a = 3 + 2;
            let b = 4;
            let c = a + b;
        }
    "#).unwrap();
    assert_eq!(3, cfg.local_ids.len());
    let mut analysis = LiveVariableAnalysis::new(&cfg);
    analysis.apply();
    println!("{:?}", analysis.in_states);
    println!("{:?}", analysis.out_states);
}
