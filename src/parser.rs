use crate::{
    peek::StringUtils,
    {Expression, Factor, Term},
};
use anyhow::{Result, anyhow};

pub trait Parser: Sized {
    fn parse(string: String) -> Result<(Self, String)>;
}

impl Parser for Expression {
    fn parse(string: String) -> Result<(Self, String)> {
        let (left, string) = Term::parse(string.trim_mut())?;
        let mut left = Expression::Term(Box::new(left));
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '+' || c == '-') {
            match string.remove(0) {
                '+' => {
                    let (right, remainder) = Term::parse(string.trim_mut())?;

                    match left {
                        Expression::Term(term) => {
                            left = Expression::Addition(term, Box::new(right));
                        }
                        _ => {
                            left = Expression::Addition(
                                Box::new(Term::Factor(Box::new(Factor::Expression(Box::new(
                                    left,
                                ))))),
                                Box::new(right),
                            );
                        }
                    }
                    string = remainder.trim_mut();
                }
                '-' => {
                    let (right, remainder) = Term::parse(string.trim_mut())?;

                    match left {
                        Expression::Term(term) => {
                            left = Expression::Subtraction(term, Box::new(right));
                        }
                        _ => {
                            left = Expression::Subtraction(
                                Box::new(Term::Factor(Box::new(Factor::Expression(Box::new(
                                    left,
                                ))))),
                                Box::new(right),
                            );
                        }
                    }

                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '+' or '-'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser for Term {
    fn parse(string: String) -> Result<(Self, String)> {
        let (left, string) = Factor::parse(string.trim_mut())?;
        let mut left = Term::Factor(Box::new(left));
        let mut string = string.trim_mut();

        while string.peek().is_some_and(|c| c == '*' || c == '/') {
            match string.remove(0) {
                '*' => {
                    let (right, remainder) = Factor::parse(string.trim_mut())?;

                    match left {
                        Term::Factor(term) => {
                            left = Term::Multiplication(term, Box::new(right));
                        }
                        _ => {
                            left = Term::Multiplication(
                                Box::new(Factor::Expression(Box::new(Expression::Term(Box::new(
                                    left,
                                ))))),
                                Box::new(right),
                            );
                        }
                    }
                    string = remainder.trim_mut();
                }
                '/' => {
                    let (right, remainder) = Factor::parse(string.trim_mut())?;

                    match left {
                        Term::Factor(term) => {
                            left = Term::Division(term, Box::new(right));
                        }
                        _ => {
                            left = Term::Division(
                                Box::new(Factor::Expression(Box::new(Expression::Term(Box::new(
                                    left,
                                ))))),
                                Box::new(right),
                            );
                        }
                    }

                    string = remainder.trim_mut();
                }
                _ => return Err(anyhow!("expected '*' or '/'")),
            }
        }

        Ok((left, string))
    }
}

impl Parser for Factor {
    fn parse(string: String) -> Result<(Self, String)> {
        let mut string = string.trim_mut();
        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0);
            let (left, remainder) = Expression::parse(string)?;

            string = remainder.trim_mut();
            if string.peek().is_some_and(|c| c == ')') {
                string.remove(0);

                return Ok((Factor::Expression(Box::new(left)), string));
            }

            return Err(anyhow!("failed to parse expression, expected ')'"));
        }
        let mut value_str = String::default();
        while string.peek().is_some_and(|c| c.is_digit(10)) {
            value_str.push(string.remove(0));
        }

        if let Ok(value) = value_str.parse::<u32>() {
            return Ok((Factor::Integer(value), string));
        }

        Err(anyhow!("failed to parse integer"))
    }
}
