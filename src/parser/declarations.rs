use crate::{
    Lexer,
    ast::{self, AstNode, LiteralAstNode},
    parser::{
        Block, Declaration, Function, Or, Parser,
        utils::{parse_identifier, parse_parameter},
    },
};
use anyhow::{Result, anyhow};

impl Parser for Function {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "fn") {
            return Err(anyhow!("Error parsing function: missing \"fn\""));
        }

        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("(")) {
            return Err(anyhow!("Error parsing function: missing \'(\'"));
        }
        lexer.pop_char();

        let mut parameters = Vec::new();
        while !lexer.peek_and(|s| s.starts_with(")")) {
            let (name, type_name) = parse_parameter(lexer)?;
            parameters.push(ast::Parameter { name, type_name });
        }
        lexer.pop_char();

        let mut return_type = "void".to_string();
        if lexer.peek_and(|s| s == ":") {
            let _ = lexer.next_token();
            match lexer.next_token() {
                Some(type_name) => {
                    return_type = type_name;
                }
                None => {
                    return Err(anyhow!("Error parsing function: missing return type"));
                }
            }
        }

        let body = Block::parse(lexer)?;
        Ok(ast::Function::new(name, parameters, return_type, body))
    }
}

impl Parser for Declaration {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if !lexer.next_token().is_some_and(|s| s == "let") {
            return Err(anyhow!("Error parsing declaration: missing \"let\""));
        }

        let identifier = parse_identifier(lexer)?;
        if !lexer.next_token().is_some_and(|s| s == ":") {
            return Err(anyhow!("Error parsing declaration: missing \":\""));
        }

        let var_type = parse_identifier(lexer)?;
        if var_type != "i32" {
            return Err(anyhow!(
                "Error parsing declaration: unsupported type \"{}\"",
                var_type
            ));
        }

        let expression = if lexer.peek_and(|s| s == "=") {
            lexer.next_token();
            Or::parse(lexer)?
        } else {
            ast::Integer::new(0)
        };

        if !lexer.next_token().is_some_and(|s| s == ";") {
            return Err(anyhow!("Error parsing declaration: missing \";\""));
        }

        Ok(ast::Declaration::new(identifier, var_type, expression))
    }
}
