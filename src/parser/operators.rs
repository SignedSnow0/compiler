use crate::{
    ast::{self, AstNode, BinaryAstNode, LiteralAstNode},
    lexer::{Lexer, Token},
    parser::{
        And, Equality, Expression, Factor, FunctionCall, Or, Parser, Relation, Term,
        utils::parse_char_lit,
    },
};
use anyhow::{Result, anyhow};

impl Parser for Or {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let mut left = And::parse(lexer)?;
        while lexer
            .consume_if(|token| token == &Token::LogicalOr)
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
            .consume_if(|token| token == &Token::LogicalAnd)
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
            lexer.consume_if(|token| token == &Token::LogicalEqual || token == &Token::NotEqual)
        {
            match token {
                Token::LogicalEqual => {
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
            token == &Token::Lesser
                || token == &Token::LesserEqual
                || token == &Token::Greater
                || token == &Token::GreaterEqual
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
            lexer.consume_if(|token| token == &Token::Plus || token == &Token::Minus)
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
            lexer.consume_if(|token| token == &Token::Multiplication || token == &Token::Division)
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
        if lexer.consume_if(|token| token == &Token::ParenL).is_some() {
            let expression = Or::parse(lexer);
            if lexer.consume_if(|token| token == &Token::ParenR).is_none() {
                return Err(anyhow!("Failed to parse expression: missing \")\""));
            }

            return expression;
        }

        match lexer.peek() {
            Token::Identifier(value) => {
                if lexer.peek_and_n(2, |token| token == &Token::ParenL) {
                    FunctionCall::parse(lexer)
                } else {
                    let ident = ast::Identifier::new(value.to_string());
                    let _ = lexer.next();
                    Ok(ident)
                }
            }
            Token::Number(value) => {
                let integer = ast::Integer::new(value.parse()?);
                let _ = lexer.next();
                Ok(integer)
            }
            Token::Quote => parse_char_lit(lexer),
            _ => Err(anyhow!(
                "Failed to parse expression: unexpected identifier \"{:?}\"",
                lexer.peek()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::AstWriter;

    #[test]
    fn test_math() -> Result<()> {
        let source = "4 + 3 / (33 - xyz) * 5";
        let mut lexer = Lexer::new(source.to_string());

        let result = {
            let mut ast = Expression::parse(&mut lexer)?;
            let mut writer = AstWriter::new();
            ast.accept(&mut writer)?;

            writer.get_string()
        };

        let expected = {
            let mut expected = ast::Addition::new(
                ast::Integer::new(4),
                ast::Division::new(
                    ast::Integer::new(3),
                    ast::Multiplication::new(
                        ast::Subtraction::new(
                            ast::Integer::new(33),
                            ast::Identifier::new("xyz".to_string()),
                        ),
                        ast::Integer::new(5),
                    ),
                ),
            );

            let mut writer = AstWriter::new();
            expected.accept(&mut writer)?;

            writer.get_string()
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_logic() -> Result<()> {
        let source = "4 * 3 < (33 - 2 && 4 >= 1) || x != 0";
        let mut lexer = Lexer::new(source.to_string());

        let result = {
            let mut ast = Or::parse(&mut lexer)?;
            let mut writer = AstWriter::new();
            ast.accept(&mut writer)?;

            writer.get_string()
        };

        let expected = {
            let mut expected = ast::Or::new(
                ast::Lesser::new(
                    ast::Multiplication::new(ast::Integer::new(4), ast::Integer::new(3)),
                    ast::And::new(
                        ast::Subtraction::new(ast::Integer::new(33), ast::Integer::new(2)),
                        ast::GreaterEqual::new(ast::Integer::new(4), ast::Integer::new(1)),
                    ),
                ),
                ast::Inequality::new(ast::Identifier::new("x".to_string()), ast::Integer::new(0)),
            );

            let mut writer = AstWriter::new();
            expected.accept(&mut writer)?;

            writer.get_string()
        };

        assert_eq!(result, expected);

        Ok(())
    }
}
