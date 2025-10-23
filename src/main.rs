use std::io::{self, BufRead};

use crate::{ast::AstNode, parser::Parser, peek::StringUtils};

mod ast;
mod parser;
mod peek;

// <expression> = <term>
//              | <expression>"+"<term>
//              | <expression>"-"<term>
pub enum Expression {
    Addition(Box<dyn AstNode<TEval = i32>>, Box<dyn AstNode<TEval = i32>>),
    Subtraction(Box<dyn AstNode<TEval = i32>>, Box<dyn AstNode<TEval = i32>>),
    Term(Box<dyn AstNode<TEval = i32>>),
}

// <term> = <factor>
//        | <term>"*"<factor>
//        | <term>"/"<factor>
pub enum Term {
    Multiplication(Box<dyn AstNode<TEval = i32>>, Box<dyn AstNode<TEval = i32>>),
    Division(Box<dyn AstNode<TEval = i32>>, Box<dyn AstNode<TEval = i32>>),
    Factor(Box<dyn AstNode<TEval = i32>>),
}

// <term> = <number>
//        | "("<expression>")"
pub enum Factor {
    Integer(i32),
    Expression(Box<dyn AstNode<TEval = i32>>),
}

fn main() {
    println!("Write an expression: ");
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    match Expression::parse(input.trim_mut()) {
        Ok((expression, remainder)) => {
            println!("Parsed expression: {}", expression);
            println!("Solution: {}", expression.eval());
        }
        Err(err) => println!("Error parsing expression: {}", err),
    }
}
