use crate::analyser::sym_resolver::SymbolResolver;
use crate::ast::AST;
use crate::ir::cfg::CFG;
use crate::ir::ir_build::IRBuilder;
use crate::ir::linear_ir::LinearIR;
use crate::ir::IRInst;
use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::{OptimizeLevel, RccError};
use crate::tests;
use crate::tests::{assert_pretty_fmt_eq, assert_fmt_eq};

mod o1_test;

#[inline]
fn expected_from_file(file_name: &str) -> String {
    tests::read_from_file(file_name, "./src/ir/tests")
}

fn ir_build_with_optimize(input: &str, opt_level: OptimizeLevel) -> Result<LinearIR, RccError> {
    let mut ir_builder = IRBuilder::new(opt_level);
    let mut lexer = Lexer::new(input);
    let mut cursor = ParseCursor::new(lexer.tokenize());
    let mut ast = AST::parse(&mut cursor)?;
    let mut sym_resolver = SymbolResolver::new();
    sym_resolver.visit_file(&mut ast.file)?;
    let ir = ir_builder.generate_ir(&mut ast)?;

    Ok(ir)
}

pub(crate) fn ir_build(input: &str) -> Result<LinearIR, RccError> {
    ir_build_with_optimize(input, OptimizeLevel::Zero)
}

fn ir_build_o1(input: &str) -> Result<LinearIR, RccError> {
    ir_build_with_optimize(input, OptimizeLevel::One)
}

fn test_cfg_iter(expected: &str, cfg: &CFG) {
    let iter: Vec<&IRInst> = cfg.iter_inst().collect();
    assert_eq!(expected, format!("{:#?}", iter));
}

#[test]
fn test_ir_builder() {
    let mut ir = ir_build("fn main() {let a = 2 + 3;}").unwrap();

    let func = ir.funcs.pop().unwrap();

    assert_eq!("main", func.name);
    let expected = expected_from_file("test_ir_builder_ir.txt");
    assert_pretty_fmt_eq(&expected, &func.insts);

    let cfg = CFG::new(func);
    assert_eq!(1, cfg.basic_blocks.len());
    let bb = cfg.basic_blocks.last().unwrap();
    assert_eq!(0, bb.id);
    assert!(bb.predecessors.is_empty());
    assert_eq!(2, bb.instructions.len());
    assert!(cfg.succ_of(0).is_empty());

    test_cfg_iter(&expected, &cfg);
}

#[test]
fn test_lit_num() {
    let ir = ir_build(
        r#"fn main() {let b: i8 = 99999999999999999999999999999;
    }"#,
    )
    .err()
    .unwrap();
    assert_eq!(
        "ParseInt(ParseIntError { kind: PosOverflow })",
        format!("{:?}", ir)
    );
}

#[test]
fn test_lit_char() {
    let ir = ir_build(r#"
        fn fff() -> char {
            'a'
        }
    "#).unwrap();
    assert_fmt_eq("[Ret(Char('a'))]", &ir.funcs.first().unwrap().insts);
}

#[test]
fn test_math_overflow() {
    let b = 0x7fffffff;
    println!("{}", b);
    let ir = ir_build(
        r#"fn main() {let b: i32 = 0x7fffffff + 9999;
    }"#,
    )
    .err()
    .unwrap();
    assert_eq!(
        "ParseInt(ParseIntError { kind: InvalidDigit })",
        format!("{:?}", ir)
    );
}

#[test]
fn test_return() {
    let ir = ir_build(
        r#"fn main() -> i32{let b = 3 + 4;
        b + 3
    }"#,
    )
    .unwrap();

    let expected = expected_from_file("test_return_ir.txt");
    assert_pretty_fmt_eq(&expected, &ir.funcs.last().unwrap().insts);
}

#[test]
fn test_return2() {
    let ir = ir_build(r#"
pub fn main() -> i32 {
    let a = 3;
    let b = 2;
    let c = "hello";
    return a + b; // add?
}

    "#).unwrap();
    let expected = expected_from_file("test_return2_ir.txt");
    assert_pretty_fmt_eq(&expected, &ir.funcs.last().unwrap().insts);
}

#[test]
fn test_if() {
    let mut ir = ir_build(
        r#"fn main() -> i32{let b = 3 + 4;
        let mut a = 0;
        if b == 7 {
            a = 5;
        } else if b != 9 {
            a = 8;
        } else if b > 100 {
            a = 1;
        } else if b < 2 {
            a = -3;
        } else if b <= 33 {
            a = 2;
        } else if b >= 50 {
            a = -22;
        } else {
            a = 333;
        }
        if b == 2 {
            return b;
        }
        a
    }"#,
    )
    .unwrap();

    let expected = expected_from_file("test_if_ir.txt");
    let func = ir.funcs.pop().unwrap();
    assert_eq!(expected, format!("{:#?}", func.insts));

    let cfg = CFG::new(func);
    assert_eq!(16, cfg.basic_blocks.len());

    let expected = expected_from_file("test_if_cfg_iter.txt");
    test_cfg_iter(&expected, &cfg);

    let expected = expected_from_file("test_if_bb.txt");
    assert_eq!(expected.trim_end(), format!("{:#?}", cfg.basic_blocks));
}

#[test]
fn test_loop() {
    let mut ir = ir_build(
        r#"
        fn main() {
            let mut a = 3;
            loop {
                a += 1;
            }
            let b = loop {
                let a = 5 + 2;
                break a;
            };
        }
    "#,
    )
    .unwrap();

    let func = ir.funcs.pop().unwrap();
    let expected = expected_from_file("test_loop_ir.txt");
    assert_eq!(expected, format!("{:#?}", func.insts));

    let cfg = CFG::new(func);
    let expected = expected_from_file("test_loop_cfg_iter.txt");
    test_cfg_iter(&expected, &cfg);

    let expected = expected_from_file("test_loop_bb.txt");
    assert_eq!(expected.trim_end(), format!("{:#?}", cfg.basic_blocks));
}

#[test]
fn test_while() {
    let ir = ir_build(
        r#"
        fn main() {
            let mut a = 3;
            while a < 10 {
                a += 1;
                if a == 5 {
                    break;
                }
            }
            while a > 1 + 2 + 3 {
            }
        }
    "#,
    )
    .unwrap();

    let expected = expected_from_file("test_while_ir.txt");
    assert_eq!(expected, format!("{:#?}", ir.funcs.last().unwrap().insts));
}

#[test]
fn fn_call_test() {
    let ir = ir_build(
        r#"
        fn foo() -> i32 {
            let a = 3 + 4;
            a
        }
        fn bar(c: i32) {
            let b = foo();
            let a = b * 2 + c;
        }
        fn baz() {
            let cc = bar(3);
            baz();
        }
    "#,
    )
    .unwrap();
    assert_eq!(3, ir.funcs.len());

    for i in 0..=2 {
        let expected_ir = expected_from_file(&format!("test_call_ir{}.txt", i));
        assert_eq!(
            expected_ir,
            format!("{:#?}", ir.funcs.get(i).unwrap().insts)
        );
    }
}
