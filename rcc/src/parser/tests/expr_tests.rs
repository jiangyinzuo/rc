use crate::ast::expr::Expr::*;
use crate::ast::expr::PathExpr;
use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};
use crate::parser::{tests};

#[test]
fn path_expr_test() {
    tests::parse_validate(
        vec!["a::b::c", "a::", "a", "::", "::a", "i8::i16"],
        vec![
            Ok(PathExpr {
                segments: vec!["a".into(), "b".into(), "c".into()],
            }),
            Err("invalid path"),
            Ok(PathExpr {
                segments: vec!["a".into()],
            }),
            Err("invalid path"),
            Err("invalid path"),
            Ok(PathExpr {
                segments: vec!["i8".into(), "i16".into()],
            }),
        ],
    );
}

#[test]
fn lit_expr_test() {
    tests::parse_validate(
        vec!["123", "'c'", r#""hello""#],
        vec![Ok(LitExpr {
            ret_type: "i32".into(),
            value: "123".to_string(),
        })],
    );
}

#[test]
fn unary_expr_test() {
    tests::parse_validate(
        vec!["!abc", "--cc::a::b", ";"],
        vec![
            Ok(Unary(UnAryExpr {
                op: UnOp::Not,
                expr: Box::new(Path(PathExpr {
                    segments: vec!["abc".into()],
                })),
            })),
            Ok(Unary(UnAryExpr {
                op: UnOp::Neg,
                expr: Box::new(Unary(UnAryExpr {
                    op: UnOp::Neg,
                    expr: Box::new(Path(PathExpr {
                        segments: vec!["cc".into(), "a".into(), "b".into()],
                    })),
                })),
            })),
            Ok(Nothing),
        ],
    )
}
