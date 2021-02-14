use crate::analyser::sym_resolver::SymbolResolver;
use crate::ast::expr::BinOperator;
use crate::ast::AST;
use crate::ir::ir_build::IRBuilder;
use crate::ir::Jump::*;
use crate::ir::Operand::I32;
use crate::ir::{IRInst, IRType, Operand, Place, IR};
use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

fn ir_build(input: &str) -> Result<IR, RccError> {
    let mut ir_builder = IRBuilder::new();
    let mut lexer = Lexer::new(input);
    let mut cursor = ParseCursor::new(lexer.tokenize());
    let mut ast = AST::parse(&mut cursor)?;
    let mut sym_resolver = SymbolResolver::new();
    sym_resolver.visit_file(&mut ast.file)?;
    let ir = ir_builder.generate_ir(&mut ast)?;

    Ok(ir)
}

#[test]
fn test_ir_builder() {
    let ir = ir_build("fn main() {let a = 2 + 3;}").unwrap();

    let insts = vec![
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local("a_2".into()),
            I32(2),
            I32(3),
        ),
        IRInst::Ret(Operand::Unit),
    ];

    assert_eq!(insts, ir.instructions);
}

#[test]
fn test_return() {
    let ir = ir_build(
        r#"fn main() -> i32{let b = 3 + 4;
        b + 3
    }"#,
    )
    .unwrap();
    let insts = vec![
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local("b_2".into()),
            I32(3),
            I32(4),
        ),
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local("$0_1".into()),
            Operand::Place(Place::local("b_2".into())),
            I32(3),
        ),
        IRInst::Ret(Operand::Place(Place::local("$0_1".into()))),
    ];
    assert_eq!(insts, ir.instructions);
}

#[test]
fn test_if() {
    let last_ir_id = 22;
    macro_rules! triple {
        ($cond:ident, $src2:literal, $load_a:literal) => {
            &mut vec![
                IRInst::jump_if_cond(
                    $cond,
                    Operand::Place(Place::local("b_2".into())),
                    Operand::I32($src2),
                    last_ir_id,
                ),
                IRInst::load_data(Place::local_mut("a_2".into()), I32($load_a)),
                IRInst::jump(last_ir_id),
            ]
        };
    }

    macro_rules! triple_reverse {
        ($cond:ident, $src1:literal, $load_a:literal) => {
            &mut vec![
                IRInst::jump_if_cond(
                    $cond,
                    Operand::I32($src1),
                    Operand::Place(Place::local("b_2".into())),
                    last_ir_id,
                ),
                IRInst::load_data(Place::local_mut("a_2".into()), I32($load_a)),
                IRInst::jump(last_ir_id),
            ]
        };
    }

    let ir = ir_build(
        r#"fn main() -> i32{let b = 3 + 4;
        let mut a = 0;
        if b == 7 {
            a = 5;
        } else if b != 9 {
            a = 8;
        } else if b > 100 {
            a = 1;
        } else if b < 2 {
            a = 3;
        } else if b <= 33 {
            a = 2;
        } else if b >= 50 {
            a = 22;
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

    let mut expected = vec![
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local("b_2".into()),
            I32(3),
            I32(4),
        ),
        IRInst::load_data(Place::local_mut("a_2".into()), I32(0)),
    ];
    expected.append(triple!(JNe, 7, 5));
    expected.append(triple!(JEq, 9, 8));
    expected.append(triple_reverse!(JGe, 100, 1));
    expected.append(triple!(JGe, 2, 3));
    expected.append(triple_reverse!(JLt, 33, 2));
    expected.append(triple!(JLt, 50, 22));
    expected.append(&mut vec![
        IRInst::load_data(Place::local_mut("a_2".into()), I32(333)),
        IRInst::jump_if_cond(JNe, Operand::Place(Place::local("b_2".into())), I32(2), 24),
        IRInst::Ret(Operand::Place(Place::local("b_2".into()))),
        IRInst::Ret(Operand::Place(Place::local_mut("a_2".into()))),
    ]);
    println!("{:#?}", ir.instructions);
    assert_eq!(expected, ir.instructions);
}

#[test]
fn test_loop() {
    let ir = ir_build(
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
    let expected = vec![
        IRInst::load_data(Place::local_mut("a_2".into()), I32(3)),
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local_mut("a_2".into()),
            Operand::Place(Place::local_mut("a_2".into())),
            I32(1),
        ),
        IRInst::jump(2),
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::local("a_4".into()),
            I32(5),
            I32(2),
        ),
        IRInst::load_data(
            Place::local("b_2".into()),
            Operand::Place(Place::local("a_4".into())),
        ),
        IRInst::jump(8),
        IRInst::jump(4),
        IRInst::Ret(Operand::Unit),
    ];

    assert_eq!(expected, ir.instructions);
}
