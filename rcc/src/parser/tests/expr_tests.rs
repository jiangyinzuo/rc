use crate::ast::expr::Expr::*;
use crate::ast::expr::RangeOp::{DotDot, DotDotEq};
use crate::ast::expr::UnOp::{Borrow, BorrowMut};
use crate::ast::expr::{
    AssignExpr, AssignOp, BinOpExpr, BinOperator, BlockExpr, CallExpr, Expr, FieldAccessExpr,
    GroupedExpr, IfExpr, PathExpr, RangeExpr, ReturnExpr, TupleExpr,
};
use crate::ast::expr::{LitNumExpr, UnAryExpr, UnOp};
use crate::ast::stmt::Stmt;
use crate::ast::types::TypeLitNum;
use crate::parser::tests::parse_validate;

#[test]
fn path_expr_test() {
    parse_validate(
        vec!["a::b::c", "a::", "a", "::", "::a", "i8::I16"],
        vec![
            Ok(PathExpr::from(vec!["a", "b", "c"])),
            Err("invalid path"),
            Ok(vec!["a"].into()),
            Err("invalid path"),
            Err("invalid path"),
            Ok(vec!["i8", "I16"].into()),
        ],
    );
}

#[test]
fn lit_expr_test() {
    parse_validate::<Expr>(
        vec!["123", "'c'", r#""hello""#],
        vec![
            Ok(Expr::LitNum(LitNumExpr {
                ret_type: TypeLitNum::I,
                value: "123".to_string(),
            })),
            Ok(Expr::LitChar('c')),
            Ok(Expr::LitStr("hello".to_string())),
        ],
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
        vec![Ok(Block(BlockExpr::from(vec![Stmt::ExprStmt(Return(
            ReturnExpr(Some(Box::new(LitNum(LitNumExpr {
                ret_type: TypeLitNum::I,
                value: "0".into(),
            })))),
        ))])))],
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
                RangeExpr::new(DotDot).lhs(LitNum(1.into())).rhs(LitNum(3.into())),
            )),
            Ok(Range(RangeExpr::new(DotDotEq).rhs(LitNum(2.into())))),
            Ok(Range(RangeExpr::new(DotDot).lhs(LitNum(3.into())))),
        ],
    );
}

#[test]
fn left_paren_test() {
    parse_validate(
        vec!["('1',)", "(1)", "(1,2)", "(1,22,)"],
        vec![
            Ok(Tuple(TupleExpr(vec![LitChar('1')]))),
            Ok(Grouped(GroupedExpr::new(LitNum(1.into())))),
            Ok(Tuple(TupleExpr(vec![LitNum(1.into()), LitNum(2.into())]))),
            Ok(Tuple(TupleExpr(vec![LitNum(1.into()), LitNum(22.into())]))),
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
                    LitNum(1.into()),
                    BinOperator::Plus,
                    BinOp(BinOpExpr::new(
                        LitNum(2.into()),
                        BinOperator::Star,
                        LitNum(4.into()),
                    )),
                )),
                BinOperator::Plus,
                LitNum(6.into()),
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
                vec![BlockExpr::new().expr_without_block(LitBool(true))],
            ))),
        ],
    );
}

#[test]
fn call_expr_test() {
    parse_validate(
        vec!["1.hello()()"],
        vec![Ok(Call(CallExpr::new(Call(CallExpr::new(FieldAccess(
            FieldAccessExpr::new(LitNum(1.into()), "hello".into()),
        ))))))],
    );
}
