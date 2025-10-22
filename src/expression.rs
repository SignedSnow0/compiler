use crate::{peek::Peek, token::Token};

#[derive(Debug)]
pub enum Exp {
    Sum(Box<Term>, Box<Term>),
    Minus(Box<Term>, Box<Term>),
    Term(Box<Term>),
}

#[derive(Debug)]
pub enum Term {
    Mul(Box<Factor>, Box<Factor>),
    Div(Box<Factor>, Box<Factor>),
    Factor(Box<Factor>),
}

#[derive(Debug)]
pub enum Factor {
    Num(u32),
    Exp(Box<Exp>),
}

impl Token for Exp {
    fn parse(string: String) -> Option<(Exp, String)> {
        println!("Parsing exp from {}", string);
        let (mut left, string) = match Term::from_string(string) {
            Some(res) => (Exp::Term(Box::new(res.0)), res.1),
            None => return None,
        };

        let mut string = string.trim_start().to_owned();
        while string.peek().is_some_and(|c| c == '+' || c == '-') {
            match string.remove(0) {
                '+' => {
                    let (right, remainder) = match Term::from_string(string) {
                        Some(res) => res,
                        None => return None,
                    };

                    println!("Parsed sum");
                    match left {
                        Exp::Term(term) => {
                            left = Exp::Sum(term, Box::new(right));
                        }
                        _ => {
                            left = Exp::Sum(
                                Box::new(Term::Factor(Box::new(Factor::Exp(Box::new(left))))),
                                Box::new(right),
                            );
                        }
                    }
                    if remainder.is_empty() {
                        return Some((left, remainder));
                    }

                    string = remainder.trim_start().to_owned();
                }
                '-' => {
                    let (right, remainder) = match Term::from_string(string) {
                        Some(res) => res,
                        None => return None,
                    };

                    println!("Parsed subtraction");
                    match left {
                        Exp::Term(term) => {
                            left = Exp::Minus(term, Box::new(right));
                        }
                        _ => {
                            left = Exp::Minus(
                                Box::new(Term::Factor(Box::new(Factor::Exp(Box::new(left))))),
                                Box::new(right),
                            );
                        }
                    }
                    if remainder.is_empty() {
                        return Some((left, remainder));
                    }

                    string = remainder.trim_start().to_owned();
                }
                _ => return None,
            }
        }

        Some((left, string))
    }

    fn starter_set() -> Vec<char> {
        Term::starter_set()
    }
}

impl Token for Term {
    fn parse(string: String) -> Option<(Self, String)>
    where
        Self: Sized,
    {
        println!("Parsing term from {}", string);
        let (mut left, string) = match Factor::from_string(string) {
            Some(res) => (Term::Factor(Box::new(res.0)), res.1),
            None => return None,
        };

        let mut string = string.trim_start().to_owned();
        while string.peek().is_some_and(|c| c == '*' || c == '/') {
            match string.remove(0) {
                '*' => {
                    let (right, remainder) = match Factor::from_string(string) {
                        Some(res) => res,
                        None => return None,
                    };
                    println!("Parsed multiplication");
                    match left {
                        Term::Factor(term) => {
                            left = Term::Mul(term, Box::new(right));
                        }
                        _ => {
                            left = Term::Mul(
                                Box::new(Factor::Exp(Box::new(Exp::Term(Box::new(left))))),
                                Box::new(right),
                            );
                        }
                    }
                    if remainder.is_empty() {
                        return Some((left, remainder));
                    }

                    string = remainder.trim_start().to_owned();
                }
                '/' => {
                    let (right, remainder) = match Factor::from_string(string) {
                        Some(res) => res,
                        None => return None,
                    };
                    println!("Parsed division");

                    match left {
                        Term::Factor(term) => {
                            left = Term::Div(term, Box::new(right));
                        }
                        _ => {
                            left = Term::Div(
                                Box::new(Factor::Exp(Box::new(Exp::Term(Box::new(left))))),
                                Box::new(right),
                            );
                        }
                    }
                    if remainder.is_empty() {
                        return Some((left, remainder));
                    }

                    string = remainder.trim_start().to_owned();
                }
                _ => return None,
            }
        }

        Some((left, string))
    }

    fn starter_set() -> Vec<char>
    where
        Self: Sized,
    {
        Factor::starter_set()
    }
}

impl Token for Factor {
    fn parse(mut string: String) -> Option<(Self, String)>
    where
        Self: Sized,
    {
        println!("Parsing factor from {}", string);
        if string.peek().is_some_and(|c| c.is_digit(10)) {
            let mut string = string;
            let mut value = "".to_owned();
            loop {
                let Some(digit) = string.peek() else {
                    break;
                };

                if digit.is_digit(10) {
                    value.push(digit);

                    string.remove(0);
                } else {
                    break;
                }
            }

            println!("Parsed num {}", value);
            match value.parse::<u32>() {
                Ok(value) => {
                    return Some((Factor::Num(value), string));
                }
                Err(_) => return None,
            }
        }

        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0).to_string();
            match Exp::from_string(string) {
                Some((exp, mut remainder)) => {
                    if remainder.peek().is_some_and(|c| c == ')') {
                        remainder.remove(0);
                        println!("Parsed exp in parenthesesis");
                        return Some((Factor::Exp(Box::new(exp)), remainder));
                    }
                    return None;
                }
                None => return None,
            }
        };

        return None;
    }

    fn starter_set() -> Vec<char>
    where
        Self: Sized,
    {
        vec!['(', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
}
