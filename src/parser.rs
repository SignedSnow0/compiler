use crate::{
    Expression, Factor, Term,
    ast::{Addition, AstNode, Division, Integer, Multiplication, Subtraction},
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

pub trait Parser<T>: Sized {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = T>>, String)>;
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
        let (mut left, string) = Factor::parse(string.trim_mut())?;
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '*' || c == '/') {
            match string.remove(0) {
                '*' => {
                    let (right, remainder) = Factor::parse(string.trim_mut())?;

                    left = Box::new(Multiplication { left, right });
                    string = remainder.trim_mut();
                }
                '/' => {
                    let (right, remainder) = Factor::parse(string.trim_mut())?;

                    left = Box::new(Division { left, right });
                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '*' or '/'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser<i32> for Factor {
    fn parse(string: String) -> Result<(Box<dyn AstNode<TEval = i32>>, String)> {
        let mut string = string.trim_mut();
        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0);
            let (left, remainder) = Expression::parse(string)?;

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
