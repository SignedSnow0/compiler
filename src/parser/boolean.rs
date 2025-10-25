use crate::{
    ast::{AstNode, BinaryAstNode, boolean::*},
    parser::{And, Expression, Factor, Not, Or, Parser, Relation, Term, utils::parse_binary_op},
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

impl Parser<i32> for Or {
    type TNext = And;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        parse_binary_op::<i32, Self::TNext, _>(
            string.trim_mut(),
            vec!['|'].as_slice(),
            |left, mut string| {
                string.remove(0);
                match string.remove(0) {
                    '|' => {
                        let (right, remainder) = Self::TNext::parse(string)?;
                        Ok((LogicalOr::new(left, right), remainder.trim_mut()))
                    }
                    _ => return Err(anyhow!("expected '|'")),
                }
            },
        )
    }
}

impl Parser<i32> for And {
    type TNext = Relation;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        parse_binary_op::<i32, Self::TNext, _>(
            string.trim_mut(),
            vec!['&'].as_slice(),
            |left, mut string| {
                string.remove(0);
                match string.remove(0) {
                    '&' => {
                        let (right, remainder) = Self::TNext::parse(string)?;
                        Ok((LogicalAnd::new(left, right), remainder.trim_mut()))
                    }
                    _ => return Err(anyhow!("expected '|'")),
                }
            },
        )
    }
}

impl Parser<i32> for Relation {
    type TNext = Expression;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        parse_binary_op::<i32, Self::TNext, _>(
            string.trim_mut(),
            vec!['>', '<'].as_slice(),
            |left, mut string| match string.remove(0) {
                '>' => {
                    if string.peek().is_some_and(|c| c == '=') {
                        string.remove(0);

                        let (right, remainder) = Expression::parse(string.trim_mut())?;
                        Ok((GreaterEqual::new(left, right), remainder.trim_mut()))
                    } else {
                        let (right, remainder) = Term::parse(string.trim_mut())?;
                        Ok((Greater::new(left, right), remainder.trim_mut()))
                    }
                }
                '<' => {
                    if string.peek().is_some_and(|c| c == '=') {
                        string.remove(0);

                        let (right, remainder) = Expression::parse(string.trim_mut())?;
                        Ok((LessEqual::new(left, right), remainder.trim_mut()))
                    } else {
                        let (right, remainder) = Term::parse(string.trim_mut())?;
                        Ok((Less::new(left, right), remainder.trim_mut()))
                    }
                }
                _ => return Err(anyhow!("expected '>' or '<'")),
            },
        )
    }
}

impl Parser<i32> for Not {
    type TNext = Factor;

    fn parse(mut string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        if let Some(c) = string.peek()
            && c == '!'
        {
            string.remove(0);
            let (value, remainder) = Factor::parse(string)?;

            return Ok((Box::new(LogicalNot { value }), remainder));
        }

        Factor::parse(string.trim_mut())
    }
}
