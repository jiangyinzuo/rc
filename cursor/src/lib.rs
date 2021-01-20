mod tests;

use std::cmp::{max, min};
use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>,
    input: &'a str,
    eaten_len: usize,
    #[cfg(debug_assertions)]
    prev: char,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            input,
            eaten_len: 0,
            #[cfg(debug_assertions)]
            prev: EOF_CHAR,
        }
    }

    /// Returns the last eaten symbol (or `'\0'` in release builds).
    /// (For debug assertions only.)
    pub fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }

        #[cfg(not(debug_assertions))]
        {
            '\0'
        }
    }

    pub fn next(&self) -> char {
        self.nth(0)
    }

    /// `nth()` start from 0
    pub fn nth(&self, n: usize) -> char {
        match self.chars.clone().nth(n) {
            Some(ch) => ch,
            None => EOF_CHAR,
        }
    }

    /// `bump_n()` start from 0
    pub fn bump_n(&mut self, n: usize) -> char {
        match self.chars.nth(n) {
            Some(c) => {
                self.eaten_len = min(self.eaten_len + n + 1, self.input.len());
                #[cfg(debug_assertions)]
                {
                    self.prev = c;
                }
                c
            }
            None => {
                self.eaten_len = self.input.len();
                #[cfg(debug_assertions)]
                {
                    self.prev = EOF_CHAR;
                }
                EOF_CHAR
            }
        }
    }

    pub fn bump(&mut self) -> char {
        self.bump_n(0)
    }

    pub fn eaten_len(&self) -> usize {
        self.eaten_len
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn eat_id(&mut self) -> usize {
        debug_assert!(is_id_start(self.next()));
        self.eat_characters(is_id_continue)
    }

    pub fn eat_whitespace(&mut self) -> usize {
        self.eat_characters(is_white_space)
    }

    pub fn eat_char_if_in(&mut self, str: &str) -> Option<char> {
        if str.contains(self.next()) {
            Some(self.bump())
        } else {
            None
        }
    }

    pub fn eat_str_if_in(&mut self, str_vec: Vec<&'a str>) -> Option<&'a str> {
        for s in str_vec {
            if self.chars.as_str().starts_with(s) {
                self.bump_n(s.len() - 1);
                return Some(s);
            }
        }
        None
    }

    pub fn eat_digits(&mut self, radix: u32) -> usize {
        debug_assert!(radix == 2 || radix == 8 || radix == 10 || radix == 16);
        let mut len = 0usize;
        while self.next().is_digit(radix) {
            self.bump();
            len += 1;
        }
        len
    }

    /// (DIGIT|_)*
    pub fn eat_digits_or_underscore(&mut self, radix: u32) -> (usize, bool) {
        debug_assert!(radix == 2 || radix == 8 || radix == 10 || radix == 16);
        let mut len = 0usize;
        let mut has_digit: bool = false;
        while self.next().is_digit(radix) || self.next() == '_' {
            has_digit = self.next() != '_';
            self.bump();
            len += 1;
        }
        (len, has_digit)
    }

    /// (DIGIT|_)* DIGIT (DIGIT|_)*
    pub fn eat_digits_with_underscore(&mut self, radix: u32) -> bool {
        let (digit_len, has_digit) = self.eat_digits_or_underscore(radix);
        digit_len > 0 && has_digit
    }

    pub fn eat_equals(&mut self, c: char, max_eat_count: usize) -> usize {
        let mut len = 0usize;
        while self.next() == c && len < max_eat_count {
            self.bump();
            len += 1;
        }
        len
    }

    /// return true if the next character is ascii character
    pub fn eat_ascii_character(&mut self) -> bool {
        if self.next() == '\\' {
            if "nrt\\0'\"".contains(self.nth(1)) {
                self.bump_n(1);
                true
            } else {
                false
            }
        } else {
            self.bump();
            true
        }
    }

    pub fn eat_characters(&mut self, ch_fn: fn(char) -> bool) -> usize {
        let mut len = 0usize;
        while ch_fn(self.next()) {
            self.bump();
            len += 1;
        }
        len
    }
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_white_space(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        '\u{0009}' // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}'
    )
}

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
    // This is XID_Start OR '_' (which formally is not a XID_Start).
    // We also add fast-path for ascii idents
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        c => (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c)),
    }
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
    // This is exactly XID_Continue.
    // We also add fast-path for ascii idents
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
        c => (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c)),
    }
}
