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
        if self.tokens.is_empty() {
            return "".to_string();
        }
        match self.stmt() {
            Ok(value) => if self.tokens.len() != 1 {
                "invalid statement".to_string()
            } else {
                value
            },
            Err(e) => e
        }
    }

    /// stmt -> id assign exp1 | exp1
    fn stmt(&mut self) -> Result<String, String> {
        let rvalue = self.exp1()?;
        if let Assign = self.tokens.back().unwrap() {
            self.tokens.pop_back();
            if let Id(_) = self.tokens.back().unwrap() {
                if let Id(id) = self.tokens.pop_back().unwrap() {
                    self.variables.insert(id, rvalue);
                    return Ok("".to_string());
                }
            }
        }
        Ok(rvalue.to_string())
    }

    /// exp1 -> exp1 Add | Sub exp2 | exp2
    fn exp1(&mut self) -> Result<i32, String> {
        let rvalue = self.exp2()?;
        match self.tokens.back().unwrap() {
            Add => {
                self.tokens.pop_back(); // pop Add
                let lvalue = self.exp1()?;
                Ok(lvalue + rvalue)
            }
            Sub => {
                self.tokens.pop_back(); // pop Sub
                let lvalue = self.exp1()?;
                Ok(lvalue - rvalue)
            }
            _ => Ok(rvalue)
        }
    }

    /// exp2 -> exp2 Multi | Div exp3 | exp3
    fn exp2(&mut self) -> Result<i32, String> {
        let rvalue = self.exp3()?;
        match self.tokens.back().unwrap() {
            Multi => {
                self.tokens.pop_back(); // pop Multi
                let lvalue = self.exp2()?;
                Ok(lvalue * rvalue)
            }
            Div => {
                self.tokens.pop_back(); // pop Div
                if rvalue == 0 {
                    Err("DIV ZERO in exp2".to_string())
                } else {
                    let lvalue = self.exp2()?;
                    Ok(lvalue / rvalue)
                }
            }
            _ => Ok(rvalue)
        }
    }

    /// exp3 -> Num | Id | OpenParen exp1 CloseParen
    fn exp3(&mut self) -> Result<i32, String> {
        match self.tokens.pop_back().unwrap() {
            CloseParen => {
                let value = self.exp1()?;
                match self.tokens.pop_back().unwrap() {
                    OpenParen => Ok(value),
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
