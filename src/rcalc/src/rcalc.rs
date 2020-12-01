use std::collections::{HashMap, VecDeque};
use super::lexer::{tokenize, Token, Token::*};

pub struct Calculator {
    variables: HashMap<String, i32>,
    tokens: VecDeque<Token>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { variables: HashMap::new(), tokens: VecDeque::new() }
    }

    pub fn interpret(&mut self, input: String) -> String {
        match tokenize(input) {
            Err(e) => return e,
            Ok(tokens) => {
                self.tokens = tokens;
            }
        }
        if self.tokens.len() <= 1 {
            return "".to_string();
        }
        let result = match self.stmt() {
            Ok(value) => value,
            Err(e) => e
        };
        if self.tokens.len() != 1 {
            "invalid statement".to_string()
        } else {
            result
        }
    }

    fn stmt(&mut self) -> Result<String, String> {
        if let Id(_) = self.tokens.front().unwrap() {
            if let Assign = self.tokens.get(1).unwrap() {
                if let Id(id) = self.tokens.pop_front().unwrap() {
                    self.tokens.pop_front(); // pop Assign
                    let value = self.exp1()?;
                    self.variables.insert(id, value);
                    return Ok("".to_string());
                }
            }
        }
        Ok(self.exp1()?.to_string())
    }

    /// exp1 -> exp2 (Add | Sub exp1) | epsilon
    fn exp1(&mut self) -> Result<i32, String> {
        let lvalue = self.exp2()?;
        match self.tokens.front().unwrap() {
            Add => {
                self.tokens.pop_front(); // pop Add
                let rvalue = self.exp1()?;
                Ok(lvalue + rvalue)
            }
            Sub => {
                self.tokens.pop_front(); // pop Sub
                let rvalue = self.exp1()?;
                Ok(lvalue - rvalue)
            }
            _ => Ok(lvalue)
        }
    }

    /// exp2 -> exp3 (Multi | Div exp2) | epsilon
    fn exp2(&mut self) -> Result<i32, String> {
        let lvalue = self.exp3()?;
        match self.tokens.front().unwrap() {
            Multi => {
                self.tokens.pop_front(); // pop Multi
                let rvalue = self.exp2()?;
                Ok(lvalue * rvalue)
            }
            Div => {
                self.tokens.pop_front(); // pop Div
                let rvalue = self.exp2()?;
                if rvalue == 0 {
                    Err("DIV ZERO in exp2".to_string())
                } else {
                    Ok(lvalue / rvalue)
                }
            }
            _ => Ok(lvalue)
        }
    }

    /// exp3 -> Num | Id | OpenParen exp1 CloseParen
    fn exp3(&mut self) -> Result<i32, String> {
        match self.tokens.pop_front().unwrap() {
            OpenParen => {
                let value = self.exp1()?;
                match self.tokens.pop_front().unwrap() {
                    CloseParen => Ok(value),
                    _ => Err("unclosed paren in exp3".to_string()),
                }
            }
            Num(n) => Ok(n),
            Id(s) => {
                if self.variables.contains_key(&s) {
                    Ok(self.variables[&s])
                } else {
                    Err(format!("variables '{}' not defined", s))
                }
            }
            tk => Err(format!("invalid token {:?} in exp3", tk))
        }
    }
}
