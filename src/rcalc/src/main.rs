mod rcalc;
mod tests;
mod lexer;

use crate::rcalc::Calculator;
use std::io::{BufRead, BufReader, Read, stdin, Write};
use std::{fs, io};

fn main() {
    let mut calculator = Calculator::new();

    loop {
        let mut input = String::new();
        input.clear();
        print!(">>> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input);
        if input.eq("exit\n") {
            break;
        }
        let result = calculator.interpret(input);
        if !result.is_empty() {
            println!("{}", result);
        }
    }
}
