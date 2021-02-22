mod lexer_tests {
    use crate::lexer::token::Token::*;
    use crate::lexer::token::{LiteralKind, LiteralKind::*, Token};
    use crate::lexer::Lexer;

    fn validate_tokenize(inputs: Vec<&str>, excepted_outputs: Vec<Vec<Token>>) {
        for (input, excepted) in inputs.iter().zip(excepted_outputs.iter()) {
            let mut lexer = Lexer::new(input);
            let res = lexer.tokenize();
            assert_eq!(*excepted, res);
        }
    }

    #[test]
    fn lex_test() {
        validate_tokenize(
            vec![
                "hello , world if  i8 0xeffff___fff 0 i81 ",
                r#"
            
            /// add a to b
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            fn main() {
                let res = add(2, 3);    
                printf("%d", res);
            }
            
            "#,
            ],
            vec![
                vec![
                    Identifier("hello"),
                    Comma,
                    Identifier("world"),
                    If,
                    Identifier("i8"),
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),

                        value: "0xeffff___fff",
                    },
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "0",
                    },
                    Identifier("i81"),
                ],
                vec![
                    Fn,
                    Identifier("add"),
                    LeftParen,
                    Identifier("a"),
                    Colon,
                    Identifier("i32"),
                    Comma,
                    Identifier("b"),
                    Colon,
                    Identifier("i32"),
                    RightParen,
                    RArrow,
                    Identifier("i32"),
                    LeftCurlyBraces,
                    Identifier("a"),
                    Plus,
                    Identifier("b"),
                    RightCurlyBraces,
                    Fn,
                    Identifier("main"),
                    LeftParen,
                    RightParen,
                    LeftCurlyBraces,
                    Let,
                    Identifier("res"),
                    Eq,
                    Identifier("add"),
                    LeftParen,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "2",
                    },
                    Comma,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "3",
                    },
                    RightParen,
                    Semi,
                    Identifier("printf"),
                    LeftParen,
                    LitString(r#""%d""#),
                    Comma,
                    Identifier("res"),
                    RightParen,
                    Semi,
                    RightCurlyBraces,
                ],
            ],
        )
    }

    #[test]
    fn eq_test() {
        validate_tokenize(
            vec!["*=+=*=%=^=!=+*%^! = =="],
            vec![vec![
                StarEq, PlusEq, StarEq, PercentEq, CaretEq, Ne, Plus, Star, Percent, Caret, Not,
                Eq, EqEq,
            ]],
        );
    }

    #[test]
    fn arrow_test() {
        validate_tokenize(
            vec!["->=>->==-==-", "---", "==-=="],
            vec![
                vec![RArrow, FatArrow, RArrow, EqEq, MinusEq, Eq, Minus],
                vec![Minus, Minus, Minus],
                vec![EqEq, MinusEq, Eq],
            ],
        );
    }

    #[test]
    fn number_literal_test() {
        validate_tokenize(
            vec!["3f32", "0o", "0b__", "12.3 1e9 0x37ffhello2  1usize"],
            vec![
                vec![Literal {
                    literal_kind: LiteralKind::f32(),
                    value: "3",
                }],
                vec![Unknown],
                vec![Unknown],
                vec![
                    Literal {
                        literal_kind: LiteralKind::float_no_suffix(),
                        value: "12.3",
                    },
                    Literal {
                        literal_kind: LiteralKind::float_no_suffix(),
                        value: "1e9",
                    },
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "0x37ff",
                    },
                    Identifier("hello2"),
                    Literal {
                        literal_kind: Integer { suffix: "usize" },
                        value: "1",
                    },
                ],
            ],
        );
    }

    #[test]
    fn string_literal_test() {
        validate_tokenize(
            vec![
                r#""hello""#,
                r#"x = "\n\\\"'\'\0\t\r""#,
                "\"\"",
                r#""hello\""#,
            ],
            vec![
                vec![LitString(r#""hello""#)],
                vec![Identifier("x"), Eq, LitString(r#""\n\\\"'\'\0\t\r""#)],
                vec![LitString("\"\"")],
                vec![Unknown],
            ],
        );
    }

    #[test]
    fn char_literal_test() {
        validate_tokenize(
            vec!["'a' '\''", "'\\", r#"'\''"#, "''", "'''"],
            vec![
                vec![
                    Literal {
                        literal_kind: Char,
                        value: "'a'",
                    },
                    Unknown,
                ],
                vec![Unknown],
                vec![Literal {
                    literal_kind: Char,
                    value: r#"'\''"#,
                }],
                vec![Unknown],
                vec![Unknown],
            ],
        );
    }

    #[test]
    fn and_or_test() {
        validate_tokenize(
            vec!["&&& |||", "& |", "&& ||", "&= |=", "1&2"],
            vec![
                vec![AndAnd, And, OrOr, Or],
                vec![And, Or],
                vec![AndAnd, OrOr],
                vec![AndEq, OrEq],
                vec![
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "1",
                    },
                    And,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "2",
                    },
                ],
            ],
        );
    }

    #[test]
    fn slash_test() {
        validate_tokenize(
            vec![
                "/**",
                "///  ///",
                "/= / //",
                "//",
                r#"/*
            
                    /*
                             */
                "#,
                r#"/*
                ///*/*/
                *// */*/"#,
            ],
            vec![
                vec![Unknown],
                vec![],
                vec![SlashEq, Slash],
                vec![],
                vec![Unknown],
                vec![],
            ],
        );
    }

    #[test]
    fn dot_test() {
        validate_tokenize(
            vec![
                ".", "..", "...", "..=", "1..2", ".2", "1.", "1.2", "a.1", "a.b", "1.a", "....",
            ],
            vec![
                vec![Dot],
                vec![DotDot],
                vec![DotDotDot],
                vec![DotDotEq],
                vec![
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "1",
                    },
                    DotDot,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "2",
                    },
                ],
                vec![
                    Dot,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "2",
                    },
                ],
                vec![Literal {
                    literal_kind: LiteralKind::float_no_suffix(),
                    value: "1.",
                }],
                vec![Literal {
                    literal_kind: LiteralKind::float_no_suffix(),
                    value: "1.2",
                }],
                vec![
                    Identifier("a"),
                    Dot,
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "1",
                    },
                ],
                vec![Identifier("a"), Dot, Identifier("b")],
                vec![
                    Literal {
                        literal_kind: LiteralKind::integer_no_suffix(),
                        value: "1",
                    },
                    Dot,
                    Identifier("a"),
                ],
                vec![DotDotDot, Dot],
            ],
        );
    }

    #[test]
    fn colon_test() {
        validate_tokenize(
            vec![":", "::", ": :"],
            vec![vec![Colon], vec![PathSep], vec![Colon, Colon]],
        );
    }

    #[test]
    fn lt_gt_test() {
        validate_tokenize(
            vec!["<  <= << <<= > >= >> >>=", "<<<"],
            vec![vec![Lt, Le, Shl, ShlEq, Gt, Ge, Shr, ShrEq], vec![Shl, Lt]],
        );
    }
}

mod token_tests {
    use crate::lexer::token::Token;
    use crate::lexer::token::Token::*;
    use std::str::FromStr;

    #[test]
    fn token_kind_test() {
        let a = Token::from_str("while").unwrap();
        assert_eq!(While, a);
        let plus = Token::from_str("+").unwrap();
        assert_eq!(Plus, plus);
    }
}
