use crate::{
    Lexer,
    ast::{self, AstNode, BinaryAstNode, LiteralAstNode},
    parser::{
        And, Equality, Expression, Factor, FunctionCall, Or, Parser, Relation, Term,
        utils::parse_identifier,
    },
};
use anyhow::{Result, anyhow};

impl Parser for Or {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = And::parse(lexer)?;
        while lexer.peek_and(|s| s == "||") {
            match lexer.peek() {
                Some(val) if val == "||" => {
                    let _ = lexer.next_token();
                    let right = And::parse(lexer)?;
                    left = ast::Or::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for And {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Equality::parse(lexer)?;
        while lexer.peek_and(|s| s == "&&") {
            match lexer.peek() {
                Some(val) if val == "&&" => {
                    let _ = lexer.next_token();
                    let right = Equality::parse(lexer)?;
                    left = ast::And::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Equality {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Relation::parse(lexer)?;
        while lexer.peek_and(|s| s == "==" || s == "!=") {
            match lexer.peek() {
                Some(val) if val == "==" => {
                    let _ = lexer.next_token();
                    let right = Relation::parse(lexer)?;
                    left = ast::Equality::new(left, right);
                }
                Some(val) if val == "!=" => {
                    let _ = lexer.next_token();
                    let right = Relation::parse(lexer)?;
                    left = ast::Inequality::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Relation {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Expression::parse(lexer)?;
        while lexer.peek_and(|s| s == "<" || s == ">" || s == "<=" || s == ">=") {
            match lexer.peek() {
                Some(val) if val == "<" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::Lesser::new(left, right);
                }
                Some(val) if val == ">" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::Greater::new(left, right);
                }
                Some(val) if val == "<=" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::LesserEqual::new(left, right);
                }
                Some(val) if val == ">=" => {
                    let _ = lexer.next_token();
                    let right = Expression::parse(lexer)?;
                    left = ast::GreaterEqual::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Expression {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Term::parse(lexer)?;
        while lexer.peek_and(|s| s == "+" || s == "-") {
            match lexer.peek() {
                Some(val) if val == "+" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = ast::Addition::new(left, right);
                }
                Some(val) if val == "-" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = ast::Subtraction::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Term {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Factor::parse(lexer)?;
        while lexer.peek_and(|s| s == "*" || s == "/") {
            match lexer.peek() {
                Some(val) if val == "*" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = ast::Multiplication::new(left, right);
                }
                Some(val) if val == "/" => {
                    let _ = lexer.next_token();
                    let right = Term::parse(lexer)?;
                    left = ast::Division::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Failed to parse expression: unexpected token \"{:?}\"",
                        lexer.peek()
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Factor {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.peek_and(|s| s.starts_with("(")) {
            lexer.pop_char();
            let expression = Or::parse(lexer);
            if !lexer.peek_and(|s| s.starts_with(")")) {
                return Err(anyhow!("Failed to parse expression: missing \')\'"));
            }
            lexer.pop_char();
            return expression;
        }

        if lexer.peek_and(|s| s.chars().next().unwrap().is_numeric()) {
            if let Some(token) = lexer.next_while(|c| c.is_numeric()) {
                Ok(ast::Integer::new(token.parse()?))
            } else {
                Err(anyhow!(
                    "Failed to parse expression: expected integer literal"
                ))
            }
        } else if lexer.peek_and(|s| s.contains("(")) {
            FunctionCall::parse(lexer)
        } else {
            let identifier = parse_identifier(lexer)?;
            Ok(ast::Identifier::new(identifier))
        }
    }
}
