mod expression;
mod peek;
mod token;

use std::io::{self, BufRead};

use crate::{expression::Exp, peek::Peek, token::Token};

fn main() {
    println!("Write an expression: ");
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut input = input.trim_start().to_owned();

    loop {
        match input.peek() {
            Some(first) => {
                if Exp::starter_set().iter().any(|c| *c == first) {
                    match Exp::parse(input) {
                        Some((expression, remainder)) => {
                            println!("Parsed: {:?}", expression);
                            input = remainder;
                        }
                        None => {
                            println!("Failed parsing expression");
                            break;
                        }
                    }
                } else {
                    input = input.trim_start().to_owned();
                }
            }
            None => break,
        }
    }
}
