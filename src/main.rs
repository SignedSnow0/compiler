use std::io::{self, BufRead};

use crate::{
    parser::{Or, Parser, Relation},
    peek::StringUtils,
};

mod ast;
mod parser;
mod peek;

fn main() {
    println!("Write an expression: ");
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    match Or::parse(input.trim_mut()) {
        Ok((expression, remainder)) => {
            println!("Parsed expression: {}", expression);
            println!("Solution: {}", expression.eval());
        }
        Err(err) => println!("Error parsing expression: {}", err),
    }
}
