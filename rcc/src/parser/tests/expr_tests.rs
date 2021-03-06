use crate::ast::expr::Expr::*;
use crate::ast::expr::RangeOp::{DotDot, DotDotEq};
use crate::ast::expr::UnOp::{Borrow, BorrowMut};
use crate::ast::expr::{
    AssignExpr, AssignOp, BinOpExpr, BinOperator, BlockExpr, CallExpr, Expr, FieldAccessExpr,
    GroupedExpr, IfExpr, LhsExpr, PathExpr, RangeExpr, ReturnExpr, TupleExpr,
};
use crate::ast::expr::{LitNumExpr, UnAryExpr, UnOp};
use crate::ast::stmt::Stmt;
use crate::ast::types::TypeLitNum;
use crate::parser::tests::{parse_validate, parse_input, expected_from_file};
use crate::rcc::RccError;
use crate::tests::assert_pretty_fmt_eq;

#[test]
fn path_expr_test() {
    parse_validate(
        vec!["a::b::c", "a::", "a", "::", "::a", "i8::I16"],
        vec![
            Ok(PathExpr::from(vec!["a", "b", "c"])),
            Err("invalid path".into()),
            Ok(vec!["a"].into()),
            Err("invalid path".into()),
            Err("invalid path".into()),
            Ok(vec!["i8", "I16"].into()),
        ],
    );
}

#[test]
fn lit_expr_test() {
    parse_validate::<Expr>(
        vec!["2f32", "123", "'c'", r#""hello""#],
        vec![
            Ok(Expr::LitNum(LitNumExpr::new(
                "2".to_string(),
                TypeLitNum::F32,
            ))),
            Ok(Expr::LitNum(LitNumExpr::new(
                "123".to_string(),
                TypeLitNum::I,
            ))),
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
            Ok(Unary(UnAryExpr::new(UnOp::Not, Path(vec!["abc"].into())))),
            Ok(Unary(UnAryExpr::new(
                UnOp::Neg,
                Unary(UnAryExpr::new(UnOp::Neg, Path(vec!["cc", "a", "b"].into()))),
            ))),
        ],
    )
}

#[test]
fn return_expr_test() {
    parse_validate(
        vec!["{ return 0;}"],
        vec![Ok(Block(BlockExpr::from(vec![Stmt::ExprStmt(Return(
            ReturnExpr(Some(Box::new(LitNum(0.into())))),
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
            LhsExpr::Path("a".into()),
            AssignOp::Eq,
            Assign(AssignExpr::new(
                LhsExpr::Path("b".into()),
                AssignOp::Eq,
                Assign(AssignExpr::new(
                    LhsExpr::Path("c".into()),
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
                    .lhs(LitNum(1.into()))
                    .rhs(LitNum(3.into())),
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
        vec!["2 3", "1+2*4+6", "1>=2<=3", "2>3+4+5", "2+32*4/5"],
        vec![
            Ok(LitNum(2.into())),
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
            Err("Chained comparison operator require parentheses".into()),
            Ok(BinOp(BinOpExpr::new(
                LitNum(2.into()),
                BinOperator::Gt,
                BinOp(BinOpExpr::new(
                    BinOp(BinOpExpr::new(
                        LitNum(3.into()),
                        BinOperator::Plus,
                        LitNum(4.into()),
                    )),
                    BinOperator::Plus,
                    LitNum(5.into()),
                )),
            ))),
            Ok(BinOp(BinOpExpr::new(
                LitNum(2.into()),
                BinOperator::Plus,
                BinOp(BinOpExpr::new(
                    BinOp(BinOpExpr::new(
                        LitNum(32.into()),
                        BinOperator::Star,
                        LitNum(4.into()),
                    )),
                    BinOperator::Slash,
                    LitNum(5.into()),
                )),
            ))),
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
                vec![BlockExpr::new(0), BlockExpr::new(0)],
            ))),
            Ok(If(IfExpr::from_exprs(
                vec![LitBool(false)],
                vec![BlockExpr::new(0).expr_without_block(LitBool(true))],
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

#[test]
fn place_expr_test() {
    let expecteds: Vec<Result<Expr, RccError>> = vec![
        Err("invalid lhs expr".into()),
        Ok(Expr::Assign(AssignExpr::new(
            LhsExpr::Deref(Box::new("a".into())),
            AssignOp::Eq,
            Expr::LitNum(4.into()),
        ))),
    ];
    parse_validate(vec!["if true {1} else {3} = 3", "*a = 4"], expecteds);
}

#[test]
fn array_expr_test() {
    let result = parse_input::<Expr>(
        r#"
        [b; 5]
    "#,
    );
    let expected = expected_from_file("array_expr_test.txt");
    assert_pretty_fmt_eq(&expected, &result.unwrap());
}
