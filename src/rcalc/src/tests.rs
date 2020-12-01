#[cfg(test)]
mod lexer_test {
    use crate::lexer::tokenize;
    use crate::lexer::Token::*;

    #[test]
    fn split_token_test() {
        let mut tokens = tokenize(String::from("hello world")).unwrap();
        assert_eq!(Id(String::from("hello")), tokens.pop_front().unwrap());
        assert_eq!(Id(String::from("world")), tokens.pop_front().unwrap());
        assert_eq!(Epsilon, tokens.pop_front().unwrap());
        assert!(tokens.is_empty());
    }
}

#[cfg(test)]
mod interpret_test {
    use crate::rcalc::Calculator;

    #[test]
    fn interpret_test() {
        let mut calculator = Calculator::new();
        let tests = [
            ("3", "1 + 2"),
            ("5", "1+2+2  "),
            ("9", "1+2*4"),
            ("11", "(1+4)*2+1"),
            ("3", "((((3))))"),
            ("", "a = 3"),
            ("", "b = 4+1*2"),
            ("", "c = (a+b)*2 - 5"),
            ("-1", "1-2*3+4    "),
            ("6", "b"),
            ("13", "c"),
            ("36", "a-b+c*a"),
        ];
        for t in tests.iter() {
            let res = calculator.interpret(t.1.to_string());
            assert_eq!(t.0.to_string(), res);
        }
    }
}