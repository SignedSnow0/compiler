use std::{collections::VecDeque, env, io::{BufRead, BufReader}};

use compiler::AstWriter;
use parser::{Program, Parser};

mod ast;
mod parser;
mod compiler;

struct Lexer {
    line_buffer: VecDeque<String>,
    reader: BufReader<std::fs::File>
}

impl Lexer {
    pub fn new(reader: BufReader<std::fs::File>) -> Self {
        Self{ line_buffer: VecDeque::default(),  reader }
    }

    pub fn pop_char(&mut self) -> Option<char> {
        if !self.ensure_buffer() {
            return None
        }

        if self.line_buffer.get(0).is_some_and(|i| { !i.is_empty() }) {
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
            return None
        }

        self.line_buffer.pop_front()
    }

    pub fn next_while<T: Fn(char) -> bool>(&mut self, predicate: T) -> Option<String> {
        if !self.ensure_buffer() {
            return None
        }

        match self.line_buffer.get_mut(0) {
            Some(item) => {
                let mut token = String::default();
                while  item.chars().next().is_some_and(|c| { predicate(c) }) {
                    token.push(item.remove(0));
                }

                if item.is_empty() {
                    self.line_buffer.remove(0);
                }

                if token.is_empty() {
                    None
                } else {
                    Some(token)
                }
            }
            None => None
        }
    }

    pub fn peek(&mut self) -> Option<&String> {
        if !self.ensure_buffer() {
            return None
        }

        self.line_buffer.get(0)
    }

    pub fn peek_and<T: Fn(&String) -> bool>(&mut self, predicate: T) -> bool {
        if let Some(item) = self.peek() {
            predicate(item)
        } else {
            false
        }
    }

    fn ensure_buffer(&mut self) -> bool {
        while self.line_buffer.is_empty() {
            let mut buffer = String::default();
            let bytes_read = match self.reader.read_line(&mut buffer) {
                Ok(x) => x,
                Err(_) => return false
            };
            if bytes_read == 0 {
                return false;
            }
            self.line_buffer = buffer.trim().split_whitespace().map(|s| { s.to_owned() }).collect();
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
        eprintln!("Failed to open input file at {}", env::args().nth(1).unwrap());
        std::process::exit(1);
    }
    let mut reader = Lexer::new(std::io::BufReader::new(file.unwrap()));
    match Program::parse(&mut reader) {
        Ok(ast) => {
            if reader.peek().is_some() {
                let remainder = reader.peek().unwrap();
                eprintln!("Error parsing program: unexpected \"{}\"", remainder);
                return;
            }
            let mut writer = AstWriter;
            let _ = ast.accept(&mut writer);
        }
        Err(msg) => {
            eprintln!(  "{}", msg);
        }
    }
}
