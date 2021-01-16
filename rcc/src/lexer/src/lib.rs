use cursor::common::*;
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
            tokens.push(self.advance_token());
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

    /// INTEGER_LITERAL :
    ///    ( DEC_LITERAL | BIN_LITERAL | OCT_LITERAL | HEX_LITERAL ) INTEGER_SUFFIX?
    /// DEC_LITERAL :
    ///    DEC_DIGIT (DEC_DIGIT|_)*
    ///
    /// BIN_LITERAL :
    ///    0b (BIN_DIGIT|_)* BIN_DIGIT (BIN_DIGIT|_)*
    ///
    /// OCT_LITERAL :
    ///    0o (OCT_DIGIT|_)* OCT_DIGIT (OCT_DIGIT|_)*
    ///
    /// HEX_LITERAL :
    ///    0x (HEX_DIGIT|_)* HEX_DIGIT (HEX_DIGIT|_)*
    ///
    /// BIN_DIGIT : [0-1]
    ///
    /// OCT_DIGIT : [0-7]
    ///
    /// DEC_DIGIT : [0-9]
    ///
    /// HEX_DIGIT : [0-9 a-f A-F]
    ///
    /// INTEGER_SUFFIX :
    ///       u8 | u16 | u32 | u64 | u128 | usize
    ///     | i8 | i16 | i32 | i64 | i128 | isize
    fn integer_or_float_literal(&mut self) -> Token {
        debug_assert!('0' <= self.cursor.next() && self.cursor.next() <= '9');
        let start = self.cursor.eaten_len();
        match self.cursor.bump() {
            '0' => {
                match self.cursor.next() {
                    // 0b | 0o | 0x
                    // Examples: 0x12ef 0b___101
                    c if matches!(c, 'b' | 'o' | 'x') => {
                        self.cursor.bump();
                        let radix: u32 = if c == 'b' {
                            2
                        } else if c == 'o' {
                            8
                        } else {
                            16
                        };
                        self.digits_with_underscore(start, radix, Self::lit_integer)
                    }

                    // 001 01.23
                    '0'..='9' => self.decimal_or_float_literal_no_prefix(start),
                    _ => LitInteger("0".to_string()),
                }
            }
            '1'..='9' => self.decimal_or_float_literal_no_prefix(start),
            _ => Unknown,
        }
    }

    /// DEC_LITERAL :
    ///    DEC_DIGIT (DEC_DIGIT|_)*
    ///
    /// FLOAT_LITERAL :
    ///       DEC_LITERAL . (not immediately followed by ., _ or an identifier)
    ///     | DEC_LITERAL FLOAT_EXPONENT
    ///     | DEC_LITERAL . DEC_LITERAL FLOAT_EXPONENT?
    ///     | DEC_LITERAL (. DEC_LITERAL)? FLOAT_EXPONENT? FLOAT_SUFFIX
    ///
    /// FLOAT_EXPONENT :
    ///     (e|E) (+|-)? (DEC_DIGIT|_)* DEC_DIGIT (DEC_DIGIT|_)*
    ///
    /// FLOAT_SUFFIX :
    ///     f32 | f64
    fn decimal_or_float_literal_no_prefix(&mut self, start: usize) -> Token {
        debug_assert!(self.cursor.prev().is_digit(10));
        // (DEC_DIGIT|_)*
        self.cursor.eat_digits_or_underscore(10);
        match self.cursor.next() {
            '.' => {
                self.cursor.bump();
                match self.cursor.next() {
                    // DEC_LITERAL FLOAT_EXPONENT?
                    '0'..='9' => {
                        // DEC_LITERAL
                        self.cursor.eat_digits_with_underscore(10);
                        // FLOAT_EXPONENT?
                        if self.cursor.next() == 'e' || self.cursor.next() == 'E' {
                            self.float_exponent(start)
                        } else {
                            self.lit_float(start)
                        }
                    }
                    // not immediately followed by ., _ or an identifier
                    c if c != '.' && c != '_' && is_id_start(c) => self.lit_integer(start),
                    _ => self.lit_integer(start),
                }
            }
            // FLOAT_EXPONENT
            'e' | 'E' => self.float_exponent(start),
            _ => self.lit_integer(start),
        }
    }

    /// FLOAT_EXPONENT :
    ///     (e|E) (+|-)? (DEC_DIGIT|_)* DEC_DIGIT (DEC_DIGIT|_)*
    fn float_exponent(&mut self, start: usize) -> Token {
        debug_assert!(self.cursor.next() == 'e' || self.cursor.next() == 'E');
        self.cursor.bump();
        self.cursor.eat_if_is_in("+-");
        self.digits_with_underscore(start, 10, Self::lit_float)
    }

    #[inline]
    fn digits_with_underscore(
        &mut self,
        start: usize,
        radix: u32,
        func: fn(&Self, usize) -> Token,
    ) -> Token {
        if self.cursor.eat_digits_with_underscore(radix) {
            func(self, start)
        } else {
            Unknown
        }
    }

    #[inline]
    fn lit_integer(&self, start: usize) -> Token {
        LitInteger(self.input[start..self.cursor.eaten_len()].to_string())
    }

    #[inline]
    fn lit_float(&self, start: usize) -> Token {
        LitFloat(self.input[start..self.cursor.eaten_len()].to_string())
    }
}
