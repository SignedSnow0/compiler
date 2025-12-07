use inkwell::context::Context;
use parser::{Parser, Program};
use std::env;

use crate::compiler::{llvmcompiler::LlvmCompiler, type_converter::TypeConverter};
use crate::lexer::Lexer;

mod ast;
mod compiler;
mod lexer;
mod parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if env::args().nth(1).is_none() {
        eprintln!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let mut lexer = Lexer::new(std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap());
    match Program::parse(&mut lexer) {
        Ok(ast) => {
            //let mut writer = AstWriter;
            //let _ = ast.accept(&mut writer);

            let context = Context::create();

            let mut type_checker = TypeConverter::new();
            ast.accept(&mut type_checker).unwrap();

            let mut compiler = LlvmCompiler::new(&context, &env::args().nth(1).unwrap());
            match ast.accept(&mut compiler) {
                Ok(_) => {
                    std::fs::write(
                        format!("{}{}", env::args().nth(1).unwrap(), ".ll"),
                        compiler.compile().unwrap(),
                    )
                    .unwrap();
                }
                Err(msg) => {
                    eprintln!("Error compiling program: {}", msg);
                }
            }
        }
        Err(msg) => {
            eprintln!(
                "Error parsing program at line {}:{}: {}",
                lexer.current_line, lexer.current_pos, msg
            );
        }
    }
}
