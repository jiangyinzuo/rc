use crate::token::Token;
use crate::token::Token::*;
use crate::Lexer;
use std::str::FromStr;

#[test]
fn lex_test() {
    let tokens = vec![
        Identifier("hello".to_string()),
        Comma,
        Identifier("world".to_string()),
        If,
        I8,
        LitInteger("0xeffff___fff".to_string()),
        LitInteger("0".to_string())
    ];
    let mut lexer = Lexer::new("hello , world if  i8 0xeffff___fff 0 ");
    let res = lexer.tokenize();
    assert_eq!(res, tokens);
}

#[test]
fn number_literal_test() {
    let inputs = ["0o", "0b__"];
    let excepteds = [vec![Unknown], vec![Unknown]];

    for (input, excepted) in inputs.iter().zip(excepteds.iter()) {
        let mut lexer = Lexer::new(input);
        let res = lexer.tokenize();
        assert_eq!(*excepted, res);
    }
}

#[test]
fn token_kind_test() {
    let a = Token::from_str("while").unwrap();
    assert_eq!(While, a);
    let plus = Token::from_str("+").unwrap();
    assert_eq!(Plus, plus);
}
