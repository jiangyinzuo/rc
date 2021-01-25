use crate::ast::expr::Expr::*;
use crate::ast::expr::RangeOp::{DotDot, DotDotEq};
use crate::ast::expr::{
    AssignExpr, AssignOp, BlockExpr, BorrowExpr, PathExpr, RangeExpr, ReturnExpr,
};
use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};
use crate::ast::stmt::Stmt;
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
            Ok(vec!["i8", "i16"].into()),
        ],
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
        vec!["!abc", "--cc::a::b"],
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
        ],
    )
}

#[test]
fn return_expr_test() {
    parse_validate(
        vec!["{ return 0;}"],
        vec![Ok(Block(BlockExpr {
            stmts: vec![
                Stmt::ExprStmt(Return(ReturnExpr(Box::new(Lit(LitExpr {
                    ret_type: "i32".into(),
                    value: "0".into(),
                }))))),
                Stmt::Semi,
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
    parse_validate(
        vec!["a = b = c &= d"],
        vec![Ok(Assign(AssignExpr::new(
            Path("a".into()),
            AssignOp::Eq,
            Assign(AssignExpr::new(
                Path("b".into()),
                AssignOp::Eq,
                Assign(AssignExpr::new(
                    Path("c".into()),
                    AssignOp::AndEq,
                    Path("d".into()),
                )),
            )),
        )))],
    );
}

#[test]
fn range_test() {
    parse_validate(
        vec!["1..3", "..=2", "3.."],
        vec![
            Ok(Range(
                RangeExpr::new(DotDot)
                    .lhs(Lit(LitExpr::lit_i32("1")))
                    .rhs(Lit(LitExpr::lit_i32("3"))),
            )),
            Ok(Range(
                RangeExpr::new(DotDotEq).rhs(Lit(LitExpr::lit_i32("2"))),
            )),
            Ok(Range(
                RangeExpr::new(DotDot).lhs(Lit(LitExpr::lit_i32("3"))),
            )),
        ],
    );
}
