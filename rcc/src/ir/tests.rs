use crate::analyser::sym_resolver::SymbolResolver;
use crate::ast::expr::BinOperator;
use crate::ast::AST;
use crate::ir::ir_build::IRBuilder;
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
    let ir = match ir_build("fn main() {let a = 2 + 3;}") {
        Ok(t) => t,
        Err(e) => {
            panic!("{:?}", e);
        }
    };

    let insts = vec![
        IRInst::bin_op(
            BinOperator::Plus,
            IRType::I32,
            Place::var("a_0".into()),
            I32(2),
            I32(3),
        ),
        IRInst::Ret(Operand::Unit),
    ];

    assert_eq!(insts, ir.instructions);
}
