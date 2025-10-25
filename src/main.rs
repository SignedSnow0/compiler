use std::io::{self, BufRead};

use crate::{
    compiler::llvmcompiler::LlvmCompiler,
    parser::{Parser, Start},
    peek::StringUtils,
};

mod ast;
mod compiler;
mod parser;
mod peek;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let file = &args[1];
        let mut input = std::fs::read_to_string(file).unwrap().trim_mut();
        while !input.is_empty() {
            match Start::parse(input.trim_mut()) {
                Ok((expression, remainder)) => {
                    println!("Parsed line: {}", expression);
                    let compiler = LlvmCompiler;
                    let code = expression.accept(&compiler);
                    println!("Generated IR:\n{}", code);
                    input = remainder.trim_mut();
                }
                Err(err) => {
                    println!("Error parsing program: {}", err);
                    break;
                }
            }
        }
        return;
    }
    println!("Write an expression: ");
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    match Start::parse(input.trim_mut()) {
        Ok((expression, _remainder)) => {
            println!("Parsed expression: {}", expression);
        }
        Err(err) => println!("Error parsing expression: {}", err),
    }
}
