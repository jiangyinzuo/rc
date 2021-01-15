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
        I8
    ];
    let mut lexer = Lexer::new("hello , world if  i8 ");
    let res = lexer.tokenize();
    assert_eq!(res, tokens);
}

#[test]
fn token_kind_test() {
    let a = Token::from_str("while").unwrap();
    assert_eq!(While, a);
    let plus = Token::from_str("+").unwrap();
    assert_eq!(Plus, plus);
}
