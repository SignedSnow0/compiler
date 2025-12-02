use crate::{
    ast::{self, AstNode, BinaryAstNode, LiteralAstNode},
    lexer::{Lexer, Token},
    parser::{And, Equality, Expression, Factor, FunctionCall, Or, Parser, Relation, Term},
};
use anyhow::{Result, anyhow};

impl Parser for Or {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = And::parse(lexer)?;
        while lexer
            .consume_if(|token| token == Token::LogicalOr)
            .is_some()
        {
            let right = And::parse(lexer)?;
            left = ast::Or::new(left, right);
        }

        Ok(left)
    }
}

impl Parser for And {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Equality::parse(lexer)?;
        while lexer
            .consume_if(|token| token == Token::LogicalAnd)
            .is_some()
        {
            let right = Equality::parse(lexer)?;
            left = ast::And::new(left, right);
        }

        Ok(left)
    }
}

impl Parser for Equality {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = Relation::parse(lexer)?;
        while let Some(token) =
            lexer.consume_if(|token| token == Token::Equal || token == Token::NotEqual)
        {
            match token {
                Token::Equal => {
                    let right = Relation::parse(lexer)?;
                    left = ast::Equality::new(left, right);
                }
                Token::NotEqual => {
                    let right = Relation::parse(lexer)?;
                    left = ast::Inequality::new(left, right)
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        token
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
        while let Some(token) = lexer.consume_if(|token| {
            token == Token::Lesser
                || token == Token::LesserEqual
                || token == Token::Greater
                || token == Token::GreaterEqual
        }) {
            match token {
                Token::Lesser => {
                    let right = Expression::parse(lexer)?;
                    left = ast::Lesser::new(left, right);
                }
                Token::LesserEqual => {
                    let right = Expression::parse(lexer)?;
                    left = ast::LesserEqual::new(left, right);
                }
                Token::Greater => {
                    let right = Expression::parse(lexer)?;
                    left = ast::Greater::new(left, right);
                }
                Token::GreaterEqual => {
                    let right = Expression::parse(lexer)?;
                    left = ast::GreaterEqual::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Error parsing expression: unexpected token \"{:?}\"",
                        token
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
        while let Some(token) =
            lexer.consume_if(|token| token == Token::Plus || token == Token::Minus)
        {
            match token {
                Token::Plus => {
                    let right = Term::parse(lexer)?;
                    left = ast::Addition::new(left, right);
                }
                Token::Minus => {
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
        while let Some(token) =
            lexer.consume_if(|token| token == Token::Multiplication || token == Token::Division)
        {
            match token {
                Token::Multiplication => {
                    let right = Term::parse(lexer)?;
                    left = ast::Multiplication::new(left, right);
                }
                Token::Division => {
                    let right = Term::parse(lexer)?;
                    left = ast::Division::new(left, right);
                }
                _ => {
                    return Err(anyhow!(
                        "Failed to parse expression: unexpected token \"{:?}\"",
                        token
                    ));
                }
            }
        }

        Ok(left)
    }
}

impl Parser for Factor {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == Token::ParenL).is_some() {
            let expression = Or::parse(lexer);
            if lexer.consume_if(|token| token == Token::ParenR).is_none() {
                return Err(anyhow!("Failed to parse expression: missing \")\""));
            }

            return expression;
        }

        if let Some(token) = lexer.consume_if(|token| token.is_identifier() || token.is_number()) {
            match token {
                Token::Identifier(value) => {
                    if lexer.peek_and(|token| token == Token::ParenL) {
                        FunctionCall::parse(lexer)
                    } else {
                        Ok(ast::Identifier::new(value))
                    }
                }
                Token::Number(value) => Ok(ast::Integer::new(value.parse()?)),
                _ => Err(anyhow!(
                    "Failed to parse expression: unexpected identifier \"{:?}\"",
                    token
                )),
            }
        } else {
            Err(anyhow!(
                "Failed to parse expression: unexpected identifier \"{:?}\"",
                lexer.peek()
            ))
        }
    }
}
