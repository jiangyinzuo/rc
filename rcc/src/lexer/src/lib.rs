use cursor::common::*;
use std::str::FromStr;

use self::token::Token::*;
use self::token::*;
use std::usize::MAX;

mod tests;
pub mod token;

struct Lexer<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
}

impl<'a: 'b, 'b> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            cursor: Cursor::new(input),
            input,
        }
    }

    pub fn tokenize(&'b mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        while !self.cursor.is_eof() {
            let token = self.advance_token();
            if token == Unknown {
                tokens.push(Unknown);
                break;
            }
            tokens.push(token);
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
            // TODO
            // '"' => self.string_literal(self.cursor.eaten_len()),
            // c if "+*%^!".contains(c) => {}
            '-' | '=' => {
                let c = self.cursor.bump();
                let i = if let Some(ch) = self.cursor.eat_if_is_in("=>") {
                    if ch == '=' {
                        1
                    } else {
                        2
                    }
                } else {
                    0
                };
                let j = if c == '-' { 0 } else { 1 };
                static TABLE: [[Token; 2]; 3] = [[Minus, Eq], [MinusEq, EqEq], [RArrow, FatArrow]];
                TABLE[i][j].clone()
            }
            c if "&|".contains(c) => {
                static TABLE: [[Token; 2]; 3] = [[And, Or], [AndAnd, OrOr], [AndEq, OrEq]];
                let mut i = self.cursor.eat_equals(c, 2) - 1;
                let j = if c == '&' { 0 } else { 1 };
                if i == 0 && self.cursor.eat_equals('=', 1) == 1 {
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
                        if self.cursor.eat_equals('=', 1) == 1 {
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
                        self.digits_with_underscore(start, radix, Self::lit_integer)
                    }

                    // 001 01.23
                    '0'..='9' => self.decimal_or_float_literal_no_prefix(start),
                    _ => LitInteger("0"),
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
                            self.lit_float(start)
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
                        self.lit_float(start)
                    }
                    // 1..2  1.a
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
    fn float_exponent(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == 'e' || self.cursor.next() == 'E');
        self.cursor.bump();
        self.cursor.eat_if_is_in("+-");
        self.digits_with_underscore(start, 10, Self::lit_float)
    }

    #[inline]
    fn digits_with_underscore(
        &'b mut self,
        start: usize,
        radix: u32,
        func: fn(&'b Self, usize) -> Token<'a>,
    ) -> Token<'a> {
        if self.cursor.eat_digits_with_underscore(radix) {
            func(self, start)
        } else {
            Unknown
        }
    }

    fn char_literal(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == '\'');
        self.cursor.bump();
        match self.cursor.bump() {
            '\\' => {
                // '\n'
                if "nrt\\0'\"".contains(self.cursor.bump()) && self.cursor.bump() == '\'' {
                    self.lit_char(start)
                } else {
                    Unknown
                }
            }
            '\'' => Unknown,
            _ => {
                if self.cursor.bump() == '\'' {
                    self.lit_char(start)
                } else {
                    Unknown
                }
            }
        }
    }

    /// TODO
    fn string_literal(&'b mut self, start: usize) -> Token<'a> {
        debug_assert!(self.cursor.next() == '"');
        // let c = '';
        Unknown
    }

    fn lit_integer(&'b self, start: usize) -> Token<'a> {
        LitInteger(&self.input[start..self.cursor.eaten_len()])
    }

    fn lit_float(&'b self, start: usize) -> Token<'a> {
        LitFloat(&self.input[start..self.cursor.eaten_len()])
    }

    fn lit_char(&'b self, start: usize) -> Token<'a> {
        LitChar(&self.input[start..self.cursor.eaten_len()])
    }
}
