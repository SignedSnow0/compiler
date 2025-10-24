use crate::{
    ast::{
        Addition, AstNode, BinaryAstNode, Division, Greater, GreaterEqual, Integer, Less,
        LessEqual, LogicalAnd, LogicalNot, LogicalOr, Multiplication, Subtraction,
    },
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

pub trait Parser<T> {
    type TNext: Parser<T>;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = T>>, String)>;
}

// <or> = <and>
//       | <or>"||"<and>
pub struct Or;

// <and> = <relation>
//       | <and>"&&"<relation>
pub struct And;

// <relation> = <expression>
//            | <relation>"<"<expression>
//            | <relation>">"<expression>
//            | <relation>"<="<expression>
//            | <relation>">="<expression>
pub struct Relation;

// <expression> = <term>
//              | <expression>"+"<term>
//              | <expression>"-"<term>
pub struct Expression;

// <term> = <not>
//        | <term>"*"<not>
//        | <term>"/"<not>
pub struct Term;

// <not> = <factor>
//       | "!"<factor>
pub struct Not;

// <term> = <number>
//        | "("<or>")"
pub struct Factor;

fn parse_binary_op<T, TNext, F>(
    string: String,
    separators: &[char],
    right_parse: F,
) -> Result<(Box<dyn AstNode<TEval = T>>, String)>
where
    TNext: Parser<T>,
    F: Fn(Box<dyn AstNode<TEval = T>>, String) -> Result<(Box<dyn AstNode<TEval = T>>, String)>,
{
    let (mut left, string) = TNext::parse(string.trim_mut())?;
    let mut string = string.trim_mut();

    while string.peek().is_some_and(|c| separators.contains(&c)) {
        let (parsed, remainder) = right_parse(left, string)?;
        left = parsed;
        string = remainder;
    }

    Ok((left, string))
}

type Start = Or;

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

impl Parser<i32> for Expression {
    type TNext = Term;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
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

impl Parser<i32> for Term {
    type TNext = Not;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
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

impl Parser<i32> for Factor {
    type TNext = Or;

    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let mut string = string.trim_mut();
        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0);
            let (left, remainder) = Self::TNext::parse(string)?;

            string = remainder.trim_mut();
            if string.peek().is_some_and(|c| c == ')') {
                string.remove(0);

                return Ok((left, string));
            }

            return Err(anyhow!("failed to parse expression, expected ')'"));
        }

        let mut value_str = String::default();
        while string.peek().is_some_and(|c| c.is_digit(10)) {
            value_str.push(string.remove(0));
        }

        if let Ok(value) = value_str.parse::<i32>() {
            return Ok((Box::new(Integer { value }), string));
        }

        Err(anyhow!("failed to parse integer"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let expression = "12 + 5 / 4".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Addition {
            left: Box::new(Integer { value: 12 }),
            right: Box::new(Division {
                left: Box::new(Integer { value: 5 }),
                right: Box::new(Integer { value: 4 }),
            }),
        });

        assert_eq!(expression.eval(), expected.eval());

        let expression = "(12 + 5) / 4".to_owned();
        let (expression, _remainder) = Start::parse(expression).unwrap();

        let expected = Box::new(Division {
            left: Box::new(Addition {
                left: Box::new(Integer { value: 12 }),
                right: Box::new(Integer { value: 5 }),
            }),
            right: Box::new(Integer { value: 4 }),
        });

        assert_eq!(expression.eval(), expected.eval());
    }
}
