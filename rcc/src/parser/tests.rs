use crate::lexer::Lexer;

use crate::parser::Parse;
use crate::parser::ParseContext;

fn parse_input<'a, T: Parse<'a>>(input: &'a str) -> Result<T, &str> {
    let mut lexer = Lexer::new(input);
    let mut cxt = ParseContext::new(lexer.tokenize());
    T::parse(&mut cxt)
}

#[cfg(test)]
mod expr_tests {
    use std::fmt::Debug;

    use crate::lexer::Lexer;
    use crate::lexer::token::LiteralKind;
    use crate::lexer::token::LiteralKind::Integer;

    use crate::ast::expr::{LitExpr, UnAryExpr, UnOp};
    use crate::ast::expr::PathExpr;
    use crate::ast::expr::Expr::*;
    use crate::parser::Parse;
    use crate::parser::ParseContext;
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
                    segments: vec!["a", "b", "c"],
                }),
                Err("invalid path"),
                Ok(PathExpr {
                    segments: vec!["a"],
                }),
                Err("invalid path"),
                Err("invalid path"),
                Ok(PathExpr {
                    segments: vec!["i8", "i16"],
                }),
            ],
        );
    }

    #[test]
    fn lit_expr_test() {
        validate_expr(
            vec!["123", "'c'", r#""hello""#],
            vec![Ok(LitExpr {
                ret_type: "i32",
                value: "123",
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
                        segments: vec!["abc"],
                    })),
                })),
                Ok(Unary(UnAryExpr {
                    op: UnOp::Neg,
                    expr: Box::new(Unary(UnAryExpr {
                        op: UnOp::Neg,
                        expr: Box::new(Path(PathExpr {
                            segments: vec!["cc", "a", "b"],
                        })),
                    })),
                })),
                Ok(Nothing),
            ],
        )
    }
}

#[cfg(test)]
mod item_tests {
    use crate::lexer::token::LiteralKind;

    use crate::ast::expr::BlockExpr;
    use crate::ast::expr::LitExpr;
    use crate::ast::item::ItemFn;
    use crate::ast::expr::Expr::Lit;

    use super::parse_input;

    #[test]
    fn item_fn_test() {
        let result = parse_input::<ItemFn>("fn main() -> i32 {0}");
        assert_eq!(Ok(ItemFn {
            ident: "main",
            ret_type: "i32",
            fn_block: Some(BlockExpr {
                exprs: vec![Lit(LitExpr { ret_type: "i32", value: "0" })]
            }),
        }), result);
    }
}

#[cfg(test)]
mod file_tests {
    use crate::lexer::token::LiteralKind;

    use crate::ast::expr::BlockExpr;
    use crate::ast::expr::LitExpr;
    use crate::ast::file::File;
    use crate::ast::item::ItemFn;
    use crate::ast::expr::Expr::Lit;
    use crate::ast::item::Item;

    use super::parse_input;

    #[test]
    fn file_test() {
        let result = parse_input::<File>("fn pi() -> f64 {3.14f64}");
        let excepted = Ok(File {
            items: vec![Item::Fn(ItemFn {
                ident: "pi",
                ret_type: "f64",
                fn_block: Some(BlockExpr {
                    exprs: vec![Lit(LitExpr { ret_type: "f64", value: "3.14" })]
                }),
            })]
        });
        assert_eq!(excepted, result);
    }
}


