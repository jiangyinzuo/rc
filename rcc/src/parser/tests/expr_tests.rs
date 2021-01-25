use crate::ast::expr::Expr::*;
use crate::ast::expr::{BlockExpr, BorrowExpr, PathExpr, ReturnExpr, AssignExpr, AssignOp};
use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};
use crate::parser::tests::parse_validate;

#[test]
fn path_expr_test() {
    parse_validate(
        vec!["a::b::c", "a::", "a", "::", "::a", "i8::i16"],
        vec![
            Ok(PathExpr::from(vec!["a", "b", "c"])),
            Err("invalid path"),
            Ok(vec!["a"].into()),
            Err("invalid path"),
            Err("invalid path"),
            Ok(vec!["i8", "i16"].into())
        ]
    );
}

#[test]
fn lit_expr_test() {
    parse_validate(
        vec!["123", "'c'", r#""hello""#],
        vec![Ok(LitExpr {
            ret_type: "i32".into(),
            value: "123".to_string(),
        })],
    );
}

#[test]
fn unary_expr_test() {
    parse_validate(
        vec!["!abc", "--cc::a::b", ";"],
        vec![
            Ok(Unary(UnAryExpr {
                op: UnOp::Not,
                expr: Box::new(Path(vec!["abc"].into())),
            })),
            Ok(Unary(UnAryExpr {
                op: UnOp::Neg,
                expr: Box::new(Unary(UnAryExpr {
                    op: UnOp::Neg,
                    expr: Box::new(Path(vec!["cc", "a", "b"].into())),
                })),
            })),
            Ok(Nothing),
        ],
    )
}

#[test]
fn return_expr_test() {
    parse_validate(
        vec!["{ return 0;}"],
        vec![Ok(Block(BlockExpr {
            exprs: vec![
                Return(ReturnExpr(Box::new(Lit(LitExpr {
                    ret_type: "i32".into(),
                    value: "0".into(),
                })))),
                Nothing,
            ],
        }))],
    );
}

#[test]
fn borrow_expr_test() {
    parse_validate(
        vec!["&&&&mut a"],
        vec![Ok(Borrow(BorrowExpr {
            borrow_cnt: 4,
            is_mut: true,
            expr: Box::new(Path(vec!["a"].into())),
        }))],
    );
}

#[test]
fn assign_op_test() {
    parse_validate(vec!["a = b = c &= d"], vec![Ok(Assign(AssignExpr::new(
        Path("a".into()), AssignOp::Eq, Assign(AssignExpr::new(
            Path("b".into()), AssignOp::Eq, Assign(AssignExpr::new(
                Path("c".into()), AssignOp::AndEq, Path("d".into())
            ))
        ))
    )))]);
}
