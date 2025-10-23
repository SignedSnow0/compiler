use std::io::{self, BufRead};

use crate::{ast::AstNode, parser::Parser, peek::StringUtils};

mod ast;
mod parser;
mod peek;

// <expression> = <term>
//              | <expression>"+"<term>
//              | <expression>"-"<term>
#[derive(Debug)]
pub enum Expression {
    Addition(Box<Term>, Box<Term>),
    Subtraction(Box<Term>, Box<Term>),
    Term(Box<Term>),
}
// <term> = <factor>
//        | <term>"*"<factor>
//        | <term>"/"<factor>
#[derive(Debug)]
pub enum Term {
    Multiplication(Box<Factor>, Box<Factor>),
    Division(Box<Factor>, Box<Factor>),
    Factor(Box<Factor>),
}

// <term> = <number>
//        | "("<expression>")"
#[derive(Debug)]
pub enum Factor {
    Integer(u32),
    Expression(Box<Expression>),
}

fn main() {
    println!("Write an expression: ");
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    match Expression::parse(input.trim_mut()) {
        Ok((expression, remainder)) => println!("{:?}\n Result: {}", expression, expression.eval()),
        Err(err) => println!("Error parsing expression: {}", err),
    }
}
