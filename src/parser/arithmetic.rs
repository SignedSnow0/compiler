use crate::{
    ast::{AstNode, BinaryAstNode, arithmetic::*},
    parser::{Expression, Not, Parser, Term, utils::parse_binary_op},
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

impl Parser for Expression {
    type TNext = Term;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)> {
        parse_binary_op::<i32, Self::TNext, _>(
            string.trim_mut(),
            vec!['+', '-'].as_slice(),
            |left, mut string| match string.remove(0) {
                '+' => {
                    let (right, remainder) = Self::TNext::parse(string)?;
                    Ok((Addition::new(left, right), remainder.trim_mut()))
                }
                '-' => {
                    let (right, remainder) = Self::TNext::parse(string)?;
                    Ok((Subtraction::new(left, right), remainder.trim_mut()))
                }
                _ => return Err(anyhow!("expected '|'")),
            },
        )
    }
}

impl Parser for Term {
    type TNext = Not;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)> {
        parse_binary_op::<i32, Self::TNext, _>(
            string.trim_mut(),
            vec!['*', '/'].as_slice(),
            |left, mut string| match string.remove(0) {
                '*' => {
                    let (right, remainder) = Self::TNext::parse(string)?;
                    Ok((Multiplication::new(left, right), remainder.trim_mut()))
                }
                '/' => {
                    let (right, remainder) = Self::TNext::parse(string)?;
                    Ok((Division::new(left, right), remainder.trim_mut()))
                }
                _ => return Err(anyhow!("expected '|'")),
            },
        )
    }
}
