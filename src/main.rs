use std::io::{self, BufRead};

use crate::{
    compiler::{NodeCompiler, llvmcompiler::LlvmCompiler},
    parser::{Parser, Start},
    peek::StringUtils,
};
use inkwell::context::Context;

mod ast;
mod compiler;
mod parser;
mod peek;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let file = &args[1];
        let mut input = std::fs::read_to_string(file).unwrap().trim_mut();
        
        let context = Context::create();
        let mut compiler = LlvmCompiler::new(file, &context);

        while !input.is_empty() {
            match Start::parse(input.trim_mut()) {
                Ok((expression, remainder)) => {
                    println!("Parsed line: {}", expression);
                    expression.accept(&mut compiler).expect("Failed to compile expression");
                    let ir = compiler.compile().expect("Failed to compile module");
                    println!("Generated IR:\n{}", ir);
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
