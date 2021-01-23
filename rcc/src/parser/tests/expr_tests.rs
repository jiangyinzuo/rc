use crate::parser::Parse;

use crate::ast::expr::Expr::*;
use crate::ast::expr::PathExpr;
use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};

use crate::parser::tests::parse_input;

fn validate_expr<'a, T: Parse<'a>>(
    inputs: std::vec::Vec<&'a str>,
    excepted_segments: Vec<Result<T, &'static str>>,
) {
    for (input, excepted) in inputs.into_iter().zip(excepted_segments) {
        let result = parse_input::<T>(input);
        match excepted {
            Ok(segments) => assert_eq!(Ok(segments), result),
            Err(s) => assert_eq!(excepted.unwrap_err(), s),
        }
    }
}

#[test]
fn path_expr_test() {
    validate_expr(
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
    validate_expr(
        vec!["123", "'c'", r#""hello""#],
        vec![Ok(LitExpr {
            ret_type: "i32".into(),
            value: "123".to_string(),
        })],
    );
}

#[test]
fn unary_expr_test() {
    validate_expr(
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

#[cfg(test)]
mod item_tests {

    use crate::ast::expr::BlockExpr;
    use crate::ast::expr::Expr::Lit;
    use crate::ast::expr::LitExpr;
    use crate::ast::item::ItemFn;

    use super::parse_input;
    use crate::ast::types::Type;

    #[test]
    fn item_fn_test() {
        let result = parse_input::<ItemFn>("fn main() -> i32 {0}");
        assert_eq!(
            Ok(ItemFn {
                name: "main".into(),
                ret_type: Type::Identifier("i32".into()),
                fn_block: Some(BlockExpr {
                    exprs: vec![Lit(LitExpr {
                        ret_type: "i32".into(),
                        value: "0".into()
                    })]
                }),
            }),
            result
        );
    }
}

#[cfg(test)]
mod file_tests {

    use crate::ast::expr::BlockExpr;
    use crate::ast::expr::Expr::Lit;
    use crate::ast::expr::LitExpr;
    use crate::ast::file::File;
    use crate::ast::item::VisItem;
    use crate::ast::item::{InnerItem, ItemFn};

    use super::parse_input;
    use crate::ast::types::Type;
    use crate::ast::Visibility::Priv;

    #[test]
    fn file_test() {
        let result = parse_input::<File>("fn pi() -> f64 {3.14f64}");
        let excepted = Ok(File {
            items: vec![VisItem::new(
                Priv,
                InnerItem::Fn(ItemFn {
                    name: "pi".into(),
                    ret_type: Type::Identifier("f64".into()),
                    fn_block: Some(BlockExpr {
                        exprs: vec![Lit(LitExpr {
                            ret_type: "f64".into(),
                            value: "3.14".into(),
                        })],
                    }),
                }),
            )],
        });
        assert_eq!(excepted, result);
    }
}
