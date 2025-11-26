use crate::{
    Lexer,
    ast::{self, AstNode},
    parser::{
        Block, Declaration, Function, Or, Parser, StructTypedef,
        utils::{parse_identifier, parse_parameter},
    },
};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

impl Parser for StructTypedef {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "struct") {
            return Err(anyhow!("Error parsing typedef: missing \"struct\""));
        }

        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s == "{") {
            return Err(anyhow!("Error parsing typedef: missing \'{{\'"));
        }
        lexer.pop_char();

        let mut fields = HashMap::new();
        while !lexer.peek_and(|s| s.starts_with("}")) {
            let (field_name, field_type) = parse_parameter(lexer)?;
            if !lexer.peek_and(|s| s == ";") {
                return Err(anyhow!("Error parsing typedef: missing \';\'"));
            }
            lexer.pop_char();

            if fields.contains_key(&field_name) {
                return Err(anyhow!(
                    "Error parsing typedef: duplicate field name \"{}\"",
                    field_name
                ));
            }

            fields.insert(field_name, field_type);
        }

        if !lexer.peek_and(|s| s == "}") {
            return Err(anyhow!("Error parsing typedef: missing \'}}\'"));
        }
        lexer.pop_char();

        Ok(ast::StructTypedef::new(name, fields))
    }
}

impl Parser for Function {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "fn") {
            return Err(anyhow!("Error parsing function: missing \"fn\""));
        }

        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("(")) {
            return Err(anyhow!("Error parsing function: missing \'(\'"));
        }
        lexer.pop_char();

        let mut parameters = HashMap::new();
        while !lexer.peek_and(|s| s.starts_with(")")) {
            let (name, p_type) = parse_parameter(lexer)?;
            if parameters.contains_key(&name) {
                return Err(anyhow!(
                    "Error parsing function: duplicate parameter name \"{}\"",
                    name
                ));
            }
            parameters.insert(name, p_type);
        }
        lexer.pop_char();

        let return_type = if lexer.peek_and(|s| s == ":") {
            let _ = lexer.next_token();
            match lexer.next_token() {
                Some(type_name) => match type_name.as_str() {
                    "i32" => Some(ast::Type::Integer32),
                    _ => Some(ast::Type::Custom(type_name)),
                },
                None => {
                    return Err(anyhow!("Error parsing function: missing return type"));
                }
            }
        } else {
            None
        };

        let body = Block::parse(lexer)?;
        Ok(ast::Function::new(name, parameters, return_type, body))
    }
}

impl Parser for Declaration {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "let") {
            return Err(anyhow!("Error parsing declaration: missing \"let\""));
        }

        let identifier = parse_identifier(lexer)?;
        let d_type = if lexer.peek_and(|s| s == ":") {
            lexer.pop_char();
            let type_name = parse_identifier(lexer)?;
            match type_name.as_str() {
                "i32" => Some(ast::Type::Integer32),
                _ => Some(ast::Type::Custom(type_name)),
            }
        } else {
            None
        };

        let expression = if lexer.peek_and(|s| s == "=") {
            lexer.next_token();
            Some(Or::parse(lexer)?)
        } else {
            None
        };

        if lexer.next_token().is_none_or(|s| s != ";") {
            return Err(anyhow!("Error parsing declaration: missing \";\""));
        }

        Ok(ast::Declaration::new(identifier, d_type, expression))
    }
}
