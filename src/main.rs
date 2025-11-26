use std::{
    collections::VecDeque,
    env,
    io::{BufRead, BufReader},
};

use inkwell::context::Context;
use parser::{Parser, Program};

use crate::compiler::llvmcompiler::LlvmCompiler;

mod ast;
mod compiler;
mod parser;

struct Lexer {
    line_buffer: VecDeque<String>,
    reader: Box<dyn BufRead>,
    pub current_line: usize,
}

impl Lexer {
    pub fn new(reader: Box<dyn BufRead>) -> Self {
        Self {
            line_buffer: VecDeque::default(),
            reader: reader,
            current_line: 0,
        }
    }

    pub fn pop_char(&mut self) -> Option<char> {
        if !self.ensure_buffer() {
            return None;
        }

        if self.line_buffer.front().is_some_and(|i| !i.is_empty()) {
            let item = self.line_buffer.get_mut(0).unwrap();
            let c = item.remove(0);
            if item.is_empty() {
                self.line_buffer.remove(0);
            }

            Some(c)
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        if !self.ensure_buffer() {
            return None;
        }

        self.line_buffer.pop_front()
    }

    pub fn next_while<T: Fn(char) -> bool>(&mut self, predicate: T) -> Option<String> {
        if !self.ensure_buffer() {
            return None;
        }

        match self.line_buffer.get_mut(0) {
            Some(item) => {
                let mut token = String::default();
                while item.chars().next().is_some_and(&predicate) {
                    token.push(item.remove(0));
                }

                if item.is_empty() {
                    self.line_buffer.remove(0);
                }

                if token.is_empty() { None } else { Some(token) }
            }
            None => None,
        }
    }

    pub fn peek(&mut self) -> Option<&String> {
        self.peek_n(0)
    }

    pub fn peek_n(&mut self, n: usize) -> Option<&String> {
        if !self.ensure_buffer() {
            return None;
        }

        self.line_buffer.get(n)
    }

    pub fn peek_and<T: Fn(&String) -> bool>(&mut self, predicate: T) -> bool {
        self.peek_and_n(1, predicate)
    }

    pub fn peek_and_n<T: Fn(&String) -> bool>(&mut self, n: usize, predicate: T) -> bool {
        for i in 0..n {
            if let Some(item) = self.peek_n(i)
                && predicate(item)
            {
                return true;
            }
        }
        false
    }

    fn ensure_buffer(&mut self) -> bool {
        while self.line_buffer.is_empty() {
            let mut buffer = String::default();
            let bytes_read = match self.reader.read_line(&mut buffer) {
                Ok(x) => x,
                Err(_) => return false,
            };
            self.current_line += 1;

            if bytes_read == 0 {
                return false;
            }
            self.line_buffer = buffer.split_whitespace().map(|s| s.to_owned()).collect();
        }
        true
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if env::args().nth(1).is_none() {
        eprintln!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::File::open(env::args().nth(1).unwrap());
    if file.is_err() {
        eprintln!(
            "Failed to open input file at {}",
            env::args().nth(1).unwrap()
        );
        std::process::exit(1);
    }
    let mut reader = Lexer::new(Box::new(std::io::BufReader::new(file.unwrap())));
    match Program::parse(&mut reader) {
        Ok(ast) => {
            if reader.peek().is_some() {
                let remainder = reader.peek().unwrap();
                eprintln!("Error parsing program: unexpected \"{}\"", remainder);
                return;
            }
            //let mut writer = AstWriter;
            //let _ = ast.accept(&mut writer);

            let context = Context::create();
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
                "Error parsing program at line {}: {}",
                reader.current_line, msg
            );
        }
    }
}
