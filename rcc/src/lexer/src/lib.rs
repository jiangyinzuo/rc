use cursor::*;
use std::str::FromStr;

use self::token::Token::*;
use self::token::*;
use crate::token::LiteralKind::*;
use std::usize::MAX;

mod tests;
pub mod token;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
}

impl<'a: 'b, 'b> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            cursor: Cursor::new(input),
            input,
        }
    }

    pub fn tokenize(&'b mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        while !self.cursor.is_eof() {
            let token = self.advance_token();

            match token {
                Unknown => {
                    tokens.push(Unknown);
                    break;
                }
                WhiteSpace | Comment => {}
                _ => tokens.push(token)
            }
        }
        tokens
    }

    fn advance_token(&'b mut self) -> Token<'a> {
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
            '\'' => self.char_literal(self.cursor.eaten_len()),
            '"' => self.string_literal(self.cursor.eaten_len()),
            c if "+*%^!".contains(c) => {
                static TABLE: [[Token; 5]; 2] = [
                    [Plus, Star, Percent, Caret, Not],
                    [PlusEq, StarEq, PercentEq, CaretEq, Ne],
                ];
                let j = "+*%^!".find(c).unwrap();
                self.cursor.bump();
                let i = if self.eat_an_equal() { 1 } else { 0 };
                TABLE[i][j].clone()
            }
            '-' | '=' => {
                static TABLE: [[Token; 2]; 3] = [[Minus, Eq], [MinusEq, EqEq], [RArrow, FatArrow]];
                let c = self.cursor.bump();
                let i = if let Some(ch) = self.cursor.eat_char_if_in("=>") {
                    if ch == '=' {
                        1
                    } else {
                        2
                    }
                } else {
                    0
                };
                let j = if c == '-' { 0 } else { 1 };
                TABLE[i][j].clone()
            }
            c if "&|".contains(c) => {
                static TABLE: [[Token; 2]; 3] = [[And, Or], [AndAnd, OrOr], [AndEq, OrEq]];
                let mut i = self.cursor.eat_equals(c, 2) - 1;
                let j = if c == '&' { 0 } else { 1 };
                if i == 0 && self.eat_an_equal() {
                    i = 2;
                }
                TABLE[i][j].clone()
            }
            '/' => {
                let slash_count = self.cursor.eat_equals('/', MAX);
                debug_assert!(slash_count >= 1);
                if slash_count == 1 {
                    match self.cursor.next() {
                        '=' => {
                            self.cursor.bump();
                            SlashEq
                        }
                        '*' => {
                            self.cursor.bump();
                            let mut comment_count = 1;
                            while comment_count > 0 {
                                match self.cursor.bump() {
                                    EOF_CHAR => return Unknown,
                                    '*' => {
                                        if self.cursor.bump() == '/' {
                                            comment_count -= 1;
                                        }
                                    }
                                    '/' => {
                                        if self.cursor.bump() == '*' {
                                            comment_count += 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Comment
                        }
                        _ => Slash,
                    }
                } else {
                    self.cursor.eat_characters(|c| c != '\n' && c != EOF_CHAR);
                    Comment
                }
            }
            c if (c == '<' || c == '>') => {
                static TABLE: [[Token; 2]; 4] = [[Lt, Shl], [Le, ShlEq], [Gt, Shr], [Ge, ShrEq]];
                let j = self.cursor.eat_equals(c, 2) - 1;
                let i = self.cursor.eat_equals('=', 1) + c as usize - '<' as usize;
                debug_assert!(i <= 3);
                debug_assert!(j <= 1);
                TABLE[i][j].clone()
            }
            '.' => {
                let dot_count = self.cursor.eat_equals('.', 3);
                match dot_count {
                    1 => Dot,
                    2 => {
                        if self.eat_an_equal() {
                            DotDotEq
                        } else {
                            DotDot
                        }
                    }
                    3 => DotDotDot,
                    _ => Unknown, // unreachable arm
                }
            }
            ':' => {
                self.cursor.bump();
                if self.cursor.next() == ':' {
                    self.cursor.bump();
                    PathSep
                } else {
                    Colon
                }
            }
            _ => {
                self.cursor.bump();
                Unknown
            }
        }
    }

    fn identifier_or_keyword(&'b mut self) -> Token<'a> {
        let len = self.cursor.eat_id();
        let str = self
            .input
            .get(self.cursor.eaten_len() - len..self.cursor.eaten_len())
            .unwrap();
        if let Ok(token) = Token::from_str(str) {
            token
        } else {
            Identifier(str)
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
    fn integer_or_float_literal(&'b mut self) -> Token<'a> {
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
                        self.digits_with_underscore(start, radix, Self::make_integer)
                    }

                    // 001 01.23
                    '0'..='9' => self.decimal_or_float_literal_no_prefix(start),
                    _ => Literal {
                        literal_kind: self.make_integer(),
                        value: "0",
                    },
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
    fn decimal_or_float_literal_no_prefix(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.prev().is_digit(10));
        // (DEC_DIGIT|_)*
        self.cursor.eat_digits_or_underscore(10);
        match self.cursor.next() {
            '.' => {
                // self.cursor.bump();
                match self.cursor.nth(1) {
                    // DEC_LITERAL FLOAT_EXPONENT?
                    '0'..='9' => {
                        #[cfg(debug_assertions)]
                            {
                                assert_eq!(self.cursor.bump(), '.');
                            }
                        #[cfg(not(debug_assertions))]
                            {
                                self.cursor.bump();
                            }

                        // DEC_LITERAL
                        self.cursor.eat_digits_with_underscore(10);
                        // FLOAT_EXPONENT?
                        if self.cursor.next() == 'e' || self.cursor.next() == 'E' {
                            self.float_exponent(start)
                        } else {
                            let end = self.cursor.eaten_len();
                            let literal_kind = self.make_float();
                            self.lit(start, end, literal_kind)
                        }
                    }
                    // not immediately followed by ., _ or an identifier
                    // 1.;
                    c if !(c == '.' || c == '_' || is_id_start(c)) => {
                        #[cfg(debug_assertions)]
                            {
                                assert_eq!(self.cursor.bump(), '.');
                            }
                        #[cfg(not(debug_assertions))]
                            {
                                self.cursor.bump();
                            }

                        let end = self.cursor.eaten_len();
                        let literal_kind = self.make_float();
                        self.lit(start, end, literal_kind)
                    }
                    // 1..2  1.a
                    _ => {
                        let end = self.cursor.eaten_len();
                        let literal_kind = self.make_integer();
                        self.lit(start, end, literal_kind)
                    }
                }
            }
            // FLOAT_EXPONENT
            'e' | 'E' => self.float_exponent(start),
            _ => {
                let end = self.cursor.eaten_len();
                let literal_kind = self.make_integer();
                self.lit(start, end, literal_kind)
            }
        }
    }

    /// FLOAT_EXPONENT :
    ///     (e|E) (+|-)? (DEC_DIGIT|_)* DEC_DIGIT (DEC_DIGIT|_)*
    fn float_exponent(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == 'e' || self.cursor.next() == 'E');
        self.cursor.bump();
        self.cursor.eat_char_if_in("+-");
        self.digits_with_underscore(start, 10, Self::make_float)
    }

    fn make_integer(&'b mut self) -> LiteralKind<'a> {
        Integer {
            suffix: self.cursor.eat_str_if_in(
                vec!["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"])
                .unwrap_or("")
        }
    }

    fn make_float(&'b mut self) -> LiteralKind<'a> {
        Float {
            suffix: self.cursor.eat_str_if_in(
                vec!["f32", "f64"]).unwrap_or("")
        }
    }

    fn digits_with_underscore(&'b mut self, start: usize, radix: u32,
                              func: fn(&'b mut Self) -> LiteralKind<'a>) -> Token<'a> {
        if self.cursor.eat_digits_with_underscore(radix) {
            let end = self.cursor.eaten_len();
            let value = &self.input[start..end];
            let literal_kind = func(self);
            Literal {
                literal_kind,
                value,
            }
        } else {
            Unknown
        }
    }

    fn char_literal(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == '\'');
        self.cursor.bump();

        // ''
        if self.cursor.next() == '\'' {
            Unknown
        } else if self.cursor.eat_ascii_character() && self.cursor.bump() == '\'' {
            self.lit(start, self.cursor.eaten_len(), Char)
        } else {
            Unknown
        }
    }

    fn string_literal(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == '"');
        self.cursor.bump();
        while self.cursor.next() != '"' && self.cursor.next() != EOF_CHAR {
            if !self.cursor.eat_ascii_character() {
                return Unknown;
            }
        }
        if self.cursor.bump() == EOF_CHAR {
            Unknown
        } else {
            self.lit(start, self.cursor.eaten_len(), String)
        }
    }

    fn lit(&'b self, start: usize, end: usize, literal_kind: LiteralKind<'a>) -> Token<'a> {
        Literal {
            literal_kind,
            value: &self.input[start..end],
        }
    }

    fn eat_an_equal(&mut self) -> bool {
        self.cursor.eat_equals('=', 1) == 1
    }
}
