mod lexer_tests {
    use crate::token::Token;
    use crate::token::Token::*;
    use crate::Lexer;

    #[test]
    fn lex_test() {
        let tokens = vec![
            Identifier("hello"),
            WhiteSpace,
            Comma,
            WhiteSpace,
            Identifier("world"),
            WhiteSpace,
            If,
            WhiteSpace,
            I8,
            WhiteSpace,
            LitInteger("0xeffff___fff"),
            WhiteSpace,
            LitInteger("0"),
            WhiteSpace,
        ];
        let mut lexer = Lexer::new("hello , world if  i8 0xeffff___fff 0 ");
        let res = lexer.tokenize();
        assert_eq!(res, tokens);
    }

    fn validate_tokenize(inputs: Vec<&str>, excepted_outputs: Vec<Vec<Token>>) {
        for (input, excepted) in inputs.iter().zip(excepted_outputs.iter()) {
            let mut lexer = Lexer::new(input);
            let res = lexer.tokenize();
            assert_eq!(*excepted, res);
        }
    }
    #[test]
    fn number_literal_test() {
        let inputs = vec!["0o", "0b__", "12.3 1e9 0x37ffhello2"];
        let excepted_outputs = vec![
            vec![Unknown],
            vec![Unknown],
            vec![
                LitFloat("12.3"),
                WhiteSpace,
                LitFloat("1e9"),
                WhiteSpace,
                LitInteger("0x37ff"),
                Identifier("hello2"),
            ],
        ];
        validate_tokenize(inputs, excepted_outputs);
    }

    #[test]
    fn char_literal_test() {
        let inputs = vec!["'a' '\''", "'\\", r#"'\''"#];
        let excepted_outputs = vec![
            vec![LitChar("'a'"), WhiteSpace, Unknown],
            vec![Unknown],
            vec![LitChar(r#"'\''"#)],
        ];
        validate_tokenize(inputs, excepted_outputs);
    }

    #[test]
    fn dot_test() {
        validate_tokenize(
            vec![
                ".", "..", "...", "..=", "1..2", ".2", "1.", "1.2", "a.1", "a.b", "1.a",
            ],
            vec![
                vec![Dot],
                vec![DotDot],
                vec![DotDotDot],
                vec![DotDotEq],
                vec![LitInteger("1"), DotDot, LitInteger("2")],
                vec![Dot, LitInteger("2")],
                vec![LitFloat("1.")],
                vec![LitFloat("1.2")],
                vec![Identifier("a"), Dot, LitInteger("1")],
                vec![Identifier("a"), Dot, Identifier("b")],
                vec![LitInteger("1"), Dot, Identifier("a")],
            ],
        );
    }

    #[test]
    fn colon_test() {
        validate_tokenize(
            vec![":", "::", ": :"],
            vec![vec![Colon], vec![PathSep], vec![Colon, WhiteSpace, Colon]],
        );
    }

    #[test]
    fn lt_gt_test() {
        validate_tokenize(
            vec!["<  <= << <<= > >= >> >>="],
            vec![vec![
                Lt, WhiteSpace, Le, WhiteSpace, Shl, WhiteSpace, ShlEq, WhiteSpace, Gt, WhiteSpace,
                Ge, WhiteSpace, Shr, WhiteSpace, ShrEq,
            ]],
        );
    }
}

mod token_tests {
    use crate::token::Token;
    use crate::token::Token::*;
    use std::str::FromStr;

    #[test]
    fn token_kind_test() {
        let a = Token::from_str("while").unwrap();
        assert_eq!(While, a);
        let plus = Token::from_str("+").unwrap();
        assert_eq!(Plus, plus);
    }
}
