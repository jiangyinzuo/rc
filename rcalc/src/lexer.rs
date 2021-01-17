use self::Token::*;
use cursor::*;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Multi,
    Div,
    Assign,
    Id(String),
    Num(i32),
    WhiteSpace,
    OpenParen,
    CloseParen,
    Unknown,
    Epsilon,
}

fn advance_token(input: &str) -> (Token, usize) {
    let mut cursor = Cursor::new(input);
    match cursor.next() {
        c if is_id_start(c) => {
            let len = cursor.eat_id();
            (Id(input[..=len - 1].to_string()), len)
        }
        '+' => (Add, 1),
        '-' => (Sub, 1),
        '*' => (Multi, 1),
        '/' => (Div, 1),
        '(' => (OpenParen, 1),
        ')' => (CloseParen, 1),
        '=' => (Assign, 1),
        c if is_white_space(c) => (WhiteSpace, cursor.eat_whitespace()),
        '0'..='9' => {
            let len = cursor.eat_digits(10);
            let num = input[..=len - 1].parse::<i32>().unwrap();
            (Num(num), len)
        }
        _ => (Unknown, 1),
    }
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: String) -> Result<VecDeque<Token>, String> {
    let mut deque = VecDeque::new();
    while !input.is_empty() {
        let (token, len) = advance_token(&input);
        if token == Unknown {
            return Err(format!("unknown character {}", input[..len].to_string()));
        } else if token != WhiteSpace {
            deque.push_back(token);
        }
        input = input[len..].parse().unwrap();
    }
    deque.push_front(Epsilon);
    Ok(deque)
}
