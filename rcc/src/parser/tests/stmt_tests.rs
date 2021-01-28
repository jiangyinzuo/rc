use crate::ast::expr::{Expr};
use crate::ast::pattern::Pattern::Identifier;
use crate::ast::pattern::{IdentifierPattern};
use crate::ast::stmt::{LetStmt, Stmt};
use crate::parser::tests::parse_validate;

#[test]
#[should_panic]
fn not_expr() {
    parse_validate(vec![";"], vec![Ok(Expr::Lit(0.into()))]);
}

#[test]
fn let_stmt_test() {
    parse_validate(
        vec!["let a=1;", "let a: i32 = 4;", "let mut bbb;"],
        vec![
            Ok(Stmt::Let(
                LetStmt::new(Identifier(IdentifierPattern::new_const("a".into())))
                    .expr(Expr::Lit(1.into())),
            )),
            Ok(Stmt::Let(
                LetStmt::new(Identifier(IdentifierPattern::new_const("a".into())))
                    ._type("i32".into())
                    .expr(Expr::Lit(4.into())),
            )),
            Ok(Stmt::Let(LetStmt::new(Identifier(
                IdentifierPattern::new_mut("bbb".into()),
            )))),
        ],
    );
}

#[test]
fn not_end_with_semicolon() {
    parse_validate(
        vec![";", "let a=1", "let a: i32 = 4", "let mut bbb"],
        vec![
            Ok(Stmt::Semi),
            Err("EOF token"),
            Err("EOF token"),
            Err("EOF token"),
        ],
    );
}
