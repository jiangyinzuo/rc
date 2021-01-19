#[cfg(test)]
mod expr_tests {

    use crate::parser::expr::lit_expr::LitExpr;
    use crate::parser::expr::path_expr::PathExpr;
    use crate::parser::expr::unary_expr::{UnAryExpr, UnOp};
    use crate::parser::expr::Expr::*;
    use crate::Parse;
    use crate::ParseContext;
    use lexer::token::LiteralKind;
    use lexer::token::LiteralKind::Integer;
    use lexer::Lexer;
    use std::fmt::Debug;

    fn parse_context(input: &str) -> ParseContext {
        let mut lexer = Lexer::new(input);
        ParseContext::new(lexer.tokenize())
    }

    fn validate_expr<'a, T: Parse<'a> + Debug + PartialEq>(
        inputs: std::vec::Vec<&'a str>,
        excepted_segments: Vec<Result<T, &'static str>>,
    ) {
        for (input, excepted) in inputs.into_iter().zip(excepted_segments) {
            let mut cxt = parse_context(input);
            let result = T::parse(&mut cxt);
            match excepted {
                Ok(segments) => assert_eq!(segments, result.unwrap()),
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
                Ok(PathExpr {
                    segments: vec!["::", "a"],
                }),
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
                literal_kind: Integer,
                value: "123",
            })],
        );
    }

    #[test]
    fn unary_expr_test() {
        validate_expr(
            vec!["!abc", "--::a::b", ";"],
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
                            segments: vec!["::", "a", "b"],
                        })),
                    })),
                })),
                Ok(Nothing),
            ],
        )
    }
}
