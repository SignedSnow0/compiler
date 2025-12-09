use crate::{
    ast::{self, AstNode},
    lexer::{Lexer, Token},
    parser::{
        Block, Declaration, Function, Or, Parser, StructTypedef,
        utils::{parse_char_lit, parse_parameter},
    },
};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

impl Parser for StructTypedef {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::Struct).is_none() {
            return Err(anyhow!("Error parsing typedef: missing \"struct\""));
        }

        let identifier = if let Some(identifier) = lexer.consume_if(|token| token.is_identifier())
            && let Token::Identifier(identifier) = identifier
        {
            identifier
        } else {
            return Err(anyhow!("Error parsing typedef: missing identifier"));
        };

        if lexer.consume_if(|token| token == &Token::CurlyL).is_none() {
            return Err(anyhow!("Error parsing typedef: missing \'{{\'"));
        }

        let mut fields = HashMap::new();
        while lexer.peek_and(|token| token != &Token::CurlyR) {
            let (ident, ident_type) = parse_parameter(lexer)?;
            if lexer
                .consume_if(|token| token == &Token::Semicolon)
                .is_none()
            {
                return Err(anyhow!("Error parsing typedef: missing \';\'"));
            }

            if let Token::Identifier(name) = ident
                && !fields.contains_key(&name)
            {
                fields.insert(name, ident_type);
            } else {
                return Err(anyhow!("Error parsing typedef: duplicate field name"));
            }
        }
        if !lexer.consume_if(|token| token == &Token::CurlyR).is_none() {
            return Err(anyhow!("Error parsing typedef: missing \'}}\'"));
        }

        Ok(ast::StructTypedef::new(identifier, fields))
    }
}

impl Parser for Function {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::Fn).is_none() {
            return Err(anyhow!("Error parsing function: missing \"fn\""));
        }

        let identifier = if let Some(identifier) = lexer.consume_if(|token| token.is_identifier())
            && let Token::Identifier(identifier) = identifier
        {
            identifier
        } else {
            return Err(anyhow!(
                "Failed to parse function declaration: missing identifier"
            ));
        };

        if lexer.consume_if(|token| token == &Token::ParenL).is_none() {
            return Err(anyhow!("Error parsing function: missing \'(\'"));
        }

        let mut parameters = HashMap::new();
        while lexer.peek_and(|token| token != &Token::ParenR) {
            let (ident, ident_type) = parse_parameter(lexer)?;
            if let Token::Identifier(name) = ident
                && !parameters.contains_key(&name)
            {
                parameters.insert(name, ident_type);
            } else {
                return Err(anyhow!(
                    "Error parsing function definition: duplicate parameter name"
                ));
            }

            let _ = lexer.consume_if(|token| token == &Token::Comma);
        }
        let _ = lexer.next();

        let return_type = if lexer.consume_if(|token| token == &Token::Colon).is_some() {
            match lexer.next() {
                Token::Identifier(type_name) => match type_name.as_str() {
                    "i32" => Some(ast::Type::Integer32),
                    "b8" => Some(ast::Type::Boolean8),
                    "c8" => Some(ast::Type::Char8),
                    _ => {
                        return Err(anyhow!(
                            "Error parsing function declaration: invalid return type"
                        ));
                    }
                },
                _ => None,
            }
        } else {
            None
        };

        let body = Block::parse(lexer)?;
        Ok(ast::Function::new(
            identifier,
            parameters,
            return_type,
            body,
        ))
    }
}

impl Parser for Declaration {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::Let).is_none() {
            return Err(anyhow!("Error parsing declaration: missing \"let\""));
        }

        let identifier = if let Some(identifier) = lexer.consume_if(|token| token.is_identifier())
            && let Token::Identifier(identifier) = identifier
        {
            identifier
        } else {
            return Err(anyhow!(
                "Failed to parse variable declaration: missing identifier"
            ));
        };

        let declaration_type = if lexer.peek_and(|token| token == &Token::Colon) {
            let _ = lexer.next();
            match lexer.next() {
                Token::Identifier(type_name) => match type_name.as_str() {
                    "i32" => Some(ast::Type::Integer32),
                    "b8" => Some(ast::Type::Boolean8),
                    "c8" => Some(ast::Type::Char8),
                    _ => {
                        return Err(anyhow!("Error parsing variable declaration: invalid type"));
                    }
                },
                _ => None,
            }
        } else {
            None
        };

        let expression = if lexer.peek_and(|token| token == &Token::Equal) {
            let _ = lexer.next();
            Some(Or::parse(lexer)?)
        } else {
            None
        };

        if declaration_type.is_none() && expression.is_none() {
            return Err(anyhow!(
                "Error parsing variable: both type and value cannot be none"
            ));
        }

        if lexer
            .consume_if(|token| token == &Token::Semicolon)
            .is_none()
        {
            return Err(anyhow!("Error parsing declaration: missing \";\""));
        }

        Ok(ast::Declaration::new(
            identifier,
            declaration_type,
            expression,
        ))
    }
}
