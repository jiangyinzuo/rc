use crate::rcc::{RcCompiler, RccError, OptimizeLevel};
use crate::code_gen::TargetPlatform;
use std::io::Read;

fn file_path(file_name: &str) -> String {
    format!("./src/tests/{}", file_name)
}

fn test_compile(input: &str, expected_output: &str) -> Result<(), RccError> {
    let input = std::fs::File::open(file_path(input))?;
    let mut expected_output =std::fs::File::open(file_path(expected_output))?;
    let output = Vec::<u8>::new();
    let mut rcc = RcCompiler::new(TargetPlatform::Riscv32, input, output, OptimizeLevel::Zero);

    rcc.compile()?;

    let s = std::str::from_utf8(rcc.output.buffer()).unwrap();
    let mut expected = String::new();
    expected_output.read_to_string(&mut expected)?;
    assert_eq!(expected, s);
    Ok(())
}

#[test]
fn rcc_test() {
    test_compile("in1.txt", "out1.txt").unwrap();
    test_compile("in2.txt", "out2.txt").unwrap();
}
