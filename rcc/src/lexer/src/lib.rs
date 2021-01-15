use cursor::common::*;
use std::collections::VecDeque;
use std::str::FromStr;

use self::token::Token::*;
use self::token::*;

mod tests;
pub mod token;

struct Lexer<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            cursor: Cursor::new(input),
            input,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.cursor.is_eof() {
            let token_kind = self.advance_token();
            if token_kind != WhiteSpace && token_kind != Comment {
                tokens.push(token_kind);
            }
        }
        tokens
    }

    fn advance_token(&mut self) -> Token {
        match self.cursor.next() {
            c if is_white_space(c) => {
                self.cursor.eat_whitespace();
                WhiteSpace
            }
            c if is_id_start(c) => self.identifier_or_keyword(),
            c if ";,@#$?{}[]()".contains(c) => {
                self.cursor.bump();
                Token::from_str(&c.to_string()).unwrap()
            }
            '0'..='9' => self.integer_or_float_literal(),
            _ => {
                self.cursor.bump();
                Unknown
            }
        }
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let len = self.cursor.eat_id();
        let str = self
            .input
            .get(self.cursor.eaten_len() - len..self.cursor.eaten_len())
            .unwrap();
        if let Ok(token) = Token::from_str(str) {
            token
        } else {
            Identifier(str.to_string())
        }
    }

    fn integer_or_float_literal(&mut self) -> Token {
        debug_assert!('0' <= self.cursor.next() && self.cursor.next() <= '9');
        let start = self.cursor.eaten_len();
        match self.cursor.bump() {
            '0' => {
                match self.cursor.next() {
                    // 0x12ef
                    // 0b___101
                    c if matches!(c, 'b' | 'o' | 'x') => {
                        self.cursor.bump();
                        let radix: u32 = if c == 'b' {
                            2
                        } else if c == 'o' {
                            8
                        } else {
                            16
                        };
                        let (digit_len, has_digit) = self.cursor.eat_digits_or_underscore(radix);
                        // 0b
                        // 0b___
                        if digit_len == 0 || !has_digit {
                            Unknown
                        } else {
                            self.lit_integer(start)
                        }
                    }

                    // 001
                    // 01.23
                    '0'..='9' => self.decimal_or_float_literal_no_prefix(start),
                    _ => LitInteger("0".to_string()),
                }
            }
            '1'..='9' => self.decimal_or_float_literal_no_prefix(start),
            _ => Unknown,
        }
    }

    /// TODO
    fn decimal_or_float_literal_no_prefix(&mut self, start: usize) -> Token {
        debug_assert!(self.cursor.prev().is_digit(10));
        self.cursor.eat_digits_or_underscore(10);
        match self.cursor.next() {
            '.' => {
                self.cursor.bump();
                match self.cursor.next() {
                    c if c == '.' || is_id_start(c) => {
                        self.lit_integer(start)
                    },
                    _ => {
                        self.cursor.eat_digits(10);
                        self.cursor.eat_digits_or_underscore(10);
                        // TODO
                        Unknown
                    }
                }

            }
            _ => self.lit_integer(start)
        }
    }

    #[inline]
    fn lit_integer(&self, start: usize) -> Token {
        LitInteger(self.input[start..self.cursor.eaten_len()].to_string())
    }
}
