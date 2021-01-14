use crate::token::TokenKind::*;
use crate::token::{Token, TokenKind};
use crate::tokenize;
use std::str::FromStr;

#[test]
fn lex_test() {
    let tokens = vec![
        Token {
            token_kind: Identifier,
            start: 0,
            len: 5,
        },
        Token {
            token_kind: Comma,
            start: 6,
            len: 1,
        },
        Token {
            token_kind: Identifier,
            start: 8,
            len: 5,
        },
        Token {
            token_kind: If,
            start: 14,
            len: 2
        }
    ];

    let res = tokenize("hello , world if   ");
    assert_eq!(res, tokens);
}

#[test]
fn token_kind_test() {
    let a = TokenKind::from_str("while").unwrap();
    assert_eq!(While, a);
    let plus = TokenKind::from_str("+").unwrap();
    assert_eq!(Plus, plus);
}
