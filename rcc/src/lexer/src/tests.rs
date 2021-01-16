mod lexer_tests {
    use crate::token::Token;
    use crate::token::Token::*;
    use crate::Lexer;

    #[test]
    fn lex_test() {
        let tokens = vec![
            Identifier("hello".to_string()),
            WhiteSpace,
            Comma,
            WhiteSpace,
            Identifier("world".to_string()),
            WhiteSpace,
            If,
            WhiteSpace,
            I8,
            WhiteSpace,
            LitInteger("0xeffff___fff".to_string()),
            WhiteSpace,
            LitInteger("0".to_string()),
            WhiteSpace,
        ];
        let mut lexer = Lexer::new("hello , world if  i8 0xeffff___fff 0 ");
        let res = lexer.tokenize();
        assert_eq!(res, tokens);
    }

    #[test]
    fn number_literal_test() {
        let inputs = ["0o", "0b__", "12.3 1e9 0x37ffhello2"];
        let excepteds = [
            vec![Unknown],
            vec![Unknown],
            vec![
                LitFloat("12.3".to_string()),
                WhiteSpace,
                LitFloat("1e9".to_string()),
                WhiteSpace,
                LitInteger("0x37ff".to_string()),
                Identifier("hello2".to_string()),
            ],
        ];
        for (input, excepted) in inputs.iter().zip(excepteds.iter()) {
            let mut lexer = Lexer::new(input);
            let res = lexer.tokenize();
            assert_eq!(*excepted, res);
        }
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
