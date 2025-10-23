use crate::{
    ast::{
        Addition, AstNode, Division, Greater, GreaterEqual, Integer, Less, LessEqual, LogicalAnd,
        LogicalNot, LogicalOr, Multiplication, Subtraction,
    },
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

pub trait Parser<T> {
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

impl Parser<i32> for Or {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let (mut left, string) = And::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '|') {
            string.remove(0);
            match string.remove(0) {
                '|' => {
                    let (right, remainder) = And::parse(string.trim_mut())?;

                    left = Box::new(LogicalOr { left, right });
                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '|'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for And {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let (mut left, string) = Relation::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '&') {
            string.remove(0);
            match string.remove(0) {
                '&' => {
                    let (right, remainder) = Relation::parse(string.trim_mut())?;

                    left = Box::new(LogicalAnd { left, right });
                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '&'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for Relation {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let (mut left, string) = Expression::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '<' || c == '>') {
            match string.remove(0) {
                '>' => {
                    if string.peek().is_some_and(|c| c == '=') {
                        string.remove(0);

                        let (right, remainder) = Expression::parse(string.trim_mut())?;

                        left = Box::new(GreaterEqual { left, right });
                        string = remainder.trim_mut();
                    } else {
                        let (right, remainder) = Term::parse(string.trim_mut())?;

                        left = Box::new(Greater { left, right });
                        string = remainder.trim_mut();
                    }
                }
                '<' => {
                    if string.peek().is_some_and(|c| c == '=') {
                        string.remove(0);

                        let (right, remainder) = Expression::parse(string.trim_mut())?;

                        left = Box::new(LessEqual { left, right });
                        string = remainder.trim_mut();
                    } else {
                        let (right, remainder) = Term::parse(string.trim_mut())?;

                        left = Box::new(Less { left, right });
                        string = remainder.trim_mut();
                    }
                }
                _ => return Err(anyhow!("expected '>' or '<'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for Expression {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let (mut left, string) = Term::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '+' || c == '-') {
            match string.remove(0) {
                '+' => {
                    let (right, remainder) = Term::parse(string.trim_mut())?;

                    left = Box::new(Addition { left, right });
                    string = remainder.trim_mut();
                }
                '-' => {
                    let (right, remainder) = Term::parse(string.trim_mut())?;

                    left = Box::new(Subtraction { left, right });
                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '+' or '-'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for Term {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let (mut left, string) = Not::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '*' || c == '/') {
            match string.remove(0) {
                '*' => {
                    let (right, remainder) = Not::parse(string.trim_mut())?;

                    left = Box::new(Multiplication { left, right });
                    string = remainder.trim_mut();
                }
                '/' => {
                    let (right, remainder) = Not::parse(string.trim_mut())?;

                    left = Box::new(Division { left, right });
                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '*' or '/'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for Not {
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
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let mut string = string.trim_mut();
        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0);
            let (left, remainder) = Or::parse(string)?;

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
