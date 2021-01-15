use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>,
    eaten_len: usize,
    #[cfg(debug_assertions)]
    prev: char,
}

const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
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
        match self.chars.clone().next() {
            Some(ch) => ch,
            None => EOF_CHAR,
        }
    }

    pub fn bump(&mut self) -> char {
        self.eaten_len += 1;
        let c = self.chars.next().unwrap();
        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }
        c
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

    pub fn eat_if_is_in(&mut self, str: &str) -> bool {
        if str.contains(self.next()) {
            self.bump();
            true
        } else {
            false
        }
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

    fn eat_characters(&mut self, ch_fn: fn(char) -> bool) -> usize {
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
