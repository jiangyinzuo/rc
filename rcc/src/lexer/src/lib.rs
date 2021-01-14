use cursor::common::*;
use std::collections::VecDeque;
use std::str::FromStr;

use self::token::TokenKind::*;
use self::token::*;

mod tests;
pub mod token;

pub fn tokenize(input: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut lexer = Lexer::new(input);

    while lexer.num_ch_eat < input.len() {
        let (token_kind, len) = lexer.advance_token();
        if token_kind != WhiteSpace && token_kind != Comment {
            tokens.push_back(Token {
                token_kind,
                start: lexer.num_ch_eat - len,
                len,
            });
        }
    }
    tokens
}

struct Lexer<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
    num_ch_eat: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let cursor = Cursor::new(input);
        Lexer {
            cursor,
            input,
            num_ch_eat: 0,
        }
    }

    fn advance_token(&mut self) -> (TokenKind, usize) {
        let eat_result = match self.cursor.next() {
            c if is_white_space(c) => (WhiteSpace, self.cursor.eat_whitespace()),
            c if is_id_start(c) => self.identifier_or_keyword(),
            c if ";,@#$?{}[]()".contains(c) => {
                self.cursor.bump();
                (TokenKind::from_str(&c.to_string()).unwrap(), 1)
            }
            _ => {
                self.cursor.bump();
                (Unknown, 1)
            }
        };
        self.num_ch_eat += eat_result.1;
        eat_result
    }

    fn identifier_or_keyword(&mut self) -> (TokenKind, usize) {
        let len = self.cursor.eat_id();
        let str = self
            .input
            .get(self.num_ch_eat..self.num_ch_eat + len)
            .unwrap();
        if let Ok(token) = TokenKind::from_str(str) {
            (token, len)
        } else {
            (Identifier, len)
        }
    }
}
