use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>,
}

const CHAR_EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
        }
    }

    pub fn next(&self) -> char {
        match self.chars.clone().next() {
            Some(ch) => ch,
            None => CHAR_EOF,
        }
    }

    pub fn bump(&mut self) -> char {
        self.chars.next().unwrap()
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

    pub fn eat_digit(&mut self) -> usize {
        self.eat_characters(|c| '0' <= c && c <= '9')
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

    match c {
        // Usual ASCII suspects
        | '\u{0009}' // \t
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
        | '\u{2029}' // PARAGRAPH SEPARATOR
        => true,
        _ => false,
    }
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
