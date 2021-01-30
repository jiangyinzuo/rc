use crate::ast::expr::Expr;
use crate::ast::pattern::IdentifierPattern;
use crate::ast::pattern::Pattern::Identifier;
use crate::ast::stmt::{LetStmt, Stmt};
use crate::parser::stmt::{parse_stmt_or_expr_without_block, StmtOrExpr};
use crate::parser::tests::{get_parser, parse_validate};
use crate::rcc::RccError;

#[test]
#[should_panic]
fn not_expr() {
    parse_validate(vec![";"], vec![Ok(Expr::Lit(0.into()))]);
}

fn validate(inputs: Vec<&str>, outputs: Vec<Result<StmtOrExpr, RccError>>) {
    for (input, output) in inputs.iter().zip(outputs) {
        let mut cursor = get_parser(input);
        let result = parse_stmt_or_expr_without_block(&mut cursor);
        assert_eq!(result, output);
    }
}

#[test]
fn let_stmt_test() {
    let inputs = vec!["let a=1;", "let a: i32 = 4;", "let mut bbb;"];
    let outputs = vec![
        Ok(StmtOrExpr::Stmt(Stmt::Let(
            LetStmt::new(Identifier(IdentifierPattern::new_const("a".into())))
                .expr(Expr::Lit(1.into())),
        ))),
        Ok(StmtOrExpr::Stmt(Stmt::Let(
            LetStmt::new(Identifier(IdentifierPattern::new_const("a".into())))
                ._type("i32".into())
                .expr(Expr::Lit(4.into())),
        ))),
        Ok(StmtOrExpr::Stmt(Stmt::Let(LetStmt::new(Identifier(
            IdentifierPattern::new_mut("bbb".into()),
        ))))),
    ];
    validate(inputs, outputs);
}

#[test]
fn not_end_with_semicolon() {
    validate(
        vec![";", "let a=1", "let a: i32 = 4", "let mut bbb"],
        vec![
            Ok(StmtOrExpr::Stmt(Stmt::Semi)),
            Err("EOF token".into()),
            Err("EOF token".into()),
            Err("EOF token".into()),
        ],
    );
}
