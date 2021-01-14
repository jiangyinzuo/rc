use std::collections::VecDeque;

use cursor::common::*;

use self::token::*;
use self::token::TokenKind::*;

pub mod token;

pub fn tokenize(input: &str) -> VecDeque<Token> {
    let mut start = 0usize;
    let mut tokens = VecDeque::new();
    let cursor = Cursor::new(input);
    let mut lexer = Lexer::new(cursor);

    while start < input.len() {
        let (token_kind, len) = lexer.advance_token();
        if token_kind != WhiteSpace && token_kind != Comment {
            tokens.push_back(Token {
                token_kind,
                start,
                len,
            });
        }
        start += len;
    }
    tokens
}

struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    fn new(cursor: Cursor<'a>) -> Self {
        Lexer { cursor }
    }

    fn advance_token(&mut self) -> (TokenKind, usize) {
        match self.cursor.next() {
            c if is_white_space(c) => (WhiteSpace, self.cursor.eat_whitespace()),
            c if is_id_start(c) => self.identifier_or_keyword(),
            c if matches!(c, ';' | ',') => {
                self.cursor.bump();
                (Unknown, 1)
                // (TokenKind::from_str(c), 1)
            }
            _ => {
                self.cursor.bump();
                (Unknown, 1)
            }
        }
    }

    fn identifier_or_keyword(&mut self) -> (TokenKind, usize) {
        let len = self.cursor.eat_id();
        (Identifier, len)
    }
}
