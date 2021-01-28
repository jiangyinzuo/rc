use crate::ast::expr::Expr::*;
use crate::ast::expr::RangeOp::{DotDot, DotDotEq};
use crate::ast::expr::UnOp::{Borrow, BorrowMut};
use crate::ast::expr::{
    AssignExpr, AssignOp, BinOpExpr, BinOperator, BlockExpr, CallExpr, FieldAccessExpr,
    GroupedExpr, IfExpr, PathExpr, RangeExpr, ReturnExpr, TupleExpr,
};
use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};
use crate::ast::stmt::Stmt;
use crate::ast::stmt::Stmt::ExprStmt;
use crate::ast::types::Type::Identifier;
use crate::parser::tests::parse_validate;
use std::env::var;

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
            ret_type: LitExpr::EMPTY_INT_TYPE.into(),
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
                    ret_type: LitExpr::EMPTY_INT_TYPE.into(),
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
        vec![Ok(Unary(UnAryExpr::new(
            Borrow,
            Unary(UnAryExpr::new(
                Borrow,
                Unary(UnAryExpr::new(
                    Borrow,
                    Unary(UnAryExpr::new(BorrowMut, Path("a".into()))),
                )),
            )),
        )))],
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
                RangeExpr::new(DotDot).lhs(Lit(1.into())).rhs(Lit(3.into())),
            )),
            Ok(Range(RangeExpr::new(DotDotEq).rhs(Lit(2.into())))),
            Ok(Range(RangeExpr::new(DotDot).lhs(Lit(3.into())))),
        ],
    );
}

#[test]
fn left_paren_test() {
    parse_validate(
        vec!["('1',)", "(1)", "(1,2)", "(1,22,)"],
        vec![
            Ok(Tuple(TupleExpr(vec![Lit('1'.into())]))),
            Ok(Grouped(GroupedExpr::new(Lit(1.into())))),
            Ok(Tuple(TupleExpr(vec![Lit(1.into()), Lit(2.into())]))),
            Ok(Tuple(TupleExpr(vec![Lit(1.into()), Lit(22.into())]))),
        ],
    );
}

#[test]
fn bin_op_test() {
    parse_validate(
        vec!["1+2*4+6", "1>=2<=3"],
        vec![
            Ok(BinOp(BinOpExpr::new(
                BinOp(BinOpExpr::new(
                    Lit(1.into()),
                    BinOperator::Plus,
                    BinOp(BinOpExpr::new(
                        Lit(2.into()),
                        BinOperator::Star,
                        Lit(4.into()),
                    )),
                )),
                BinOperator::Plus,
                Lit(6.into()),
            ))),
            Err("Chained comparison operator require parentheses"),
        ],
    );
}

#[test]
fn if_expr_test() {
    parse_validate(
        vec!["if true {} else {}", "if false {true}"],
        vec![
            Ok(If(IfExpr::from_exprs(
                vec![LitBool(true)],
                vec![BlockExpr::new(), BlockExpr::new()],
            ))),
            Ok(If(IfExpr::from_exprs(
                vec![LitBool(false)],
                vec![vec![LitBool(true).into()].into()],
            ))),
        ],
    );
}

#[test]
fn call_expr_test() {
    parse_validate(
        vec!["1.hello()()"],
        vec![Ok(Call(CallExpr::new(Call(CallExpr::new(FieldAccess(
            FieldAccessExpr::new(Lit(1.into()), "hello".into()),
        ))))))],
    );
}
