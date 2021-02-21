use crate::ir::cfg::CFG;
use crate::ir::tests::ir_build;
use crate::rcc::RccError;
use crate::ir::dataflow::live_variable::LiveVariableAnalysis;

fn get_cfg(input: &str) -> Result<CFG, RccError> {
    let mut ir = ir_build(input)?;
    let cfg = CFG::new(ir.funcs.pop().unwrap());
    Ok(cfg)
}

#[test]
fn one_block_test() {
    let cfg = get_cfg(r#"
        fn fooo() {
            let mut a = 3;
            a = 3 + 2;
            let b = 4;
            let c = a + b;
        }
    "#).unwrap();
    assert_eq!(3, cfg.local_infos.len());
    let mut analysis = LiveVariableAnalysis::new(&cfg);
    analysis.apply();

    assert_eq!("[BitVector { inner: [0], size: 3 }]", format!("{:?}", analysis.in_states));
    assert_eq!("[BitVector { inner: [0], size: 3 }]", format!("{:?}", analysis.out_states));
}

#[test]
fn multiple_blocks_test() {
    let cfg = get_cfg(r#"
        fn bar(x: i32, y: i32) {
        }
        fn fooo() {
            let mut a = 3;
            a = 3 + 2;
            let b = 4;
            if a > b {
                let mut c = a;
                bar(a, c);
                //c = 4;
            }
        }
    "#).unwrap();
    assert_eq!(3, cfg.local_infos.len());
    let mut analysis = LiveVariableAnalysis::new(&cfg);
    analysis.apply();
    println!("{:?}", analysis.in_states);
}
