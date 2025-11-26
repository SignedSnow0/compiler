use crate::{
    Lexer,
    ast::{self, AstNode},
    parser::{
        Assignment, Block, Declaration, FunctionCall, If, Instruction, Or, Parser, Return, While,
        utils::parse_identifier,
    },
};
use anyhow::{Result, anyhow};

impl Parser for Block {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        match lexer.peek() {
            Some(item) if item == "{" => {
                lexer.pop_char();
                let mut block = ast::Block::new();
                while !lexer.peek_and(|s| s.starts_with("}")) {
                    block.add_node(Instruction::parse(lexer)?);
                }

                if !lexer.peek_and(|s| s.starts_with("}")) {
                    return Err(anyhow!("Failed to parse block: missing '}}'"));
                }
                lexer.pop_char();

                Ok(block)
            }
            Some(item) if item == "let" => Declaration::parse(lexer),
            _ => Err(anyhow!("Unexpected token: \"{}\"", lexer.peek().unwrap())),
        }
    }
}

impl Parser for Instruction {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        match lexer.peek() {
            Some(item) if item == "let" => Declaration::parse(lexer),
            Some(item) if item == "if" => If::parse(lexer),
            Some(item) if item == "while" => While::parse(lexer),
            Some(item) if item == "return" => Return::parse(lexer),
            Some(_) => {
                if lexer.peek_and_n(2, |p| p.contains("=")) {
                    Assignment::parse(lexer)
                } else {
                    let call = FunctionCall::parse(lexer)?;
                    if lexer.next_token().is_some_and(|s| s == ";") {
                        Ok(call)
                    } else {
                        Err(anyhow!("Error parsing function call: missing \";\""))
                    }
                }
            }
            None => Err(anyhow!("Error parsing instruction: unexpected EOF")),
        }
    }
}

impl Parser for Return {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "return") {
            return Err(anyhow!(
                "Error parsing return statement: missing \"return\""
            ));
        }

        let expression = Or::parse(lexer)?;
        if lexer.next_token().is_none_or(|s| s != ";") {
            return Err(anyhow!("Error parsing return statement: missing \";\""));
        }

        Ok(ast::Return::new(expression))
    }
}

impl Parser for If {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "if") {
            return Err(anyhow!("Error parsing if statement: missing \"if\""));
        }

        let expression = Or::parse(lexer)?;
        let then_block = Block::parse(lexer)?;

        let else_block = if lexer.peek_and(|s| s == "else") {
            let _ = lexer.next_token();
            Some(Block::parse(lexer)?)
        } else {
            None
        };

        Ok(ast::If::new(expression, then_block, else_block))
    }
}

impl Parser for While {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.next_token().is_none_or(|s| s != "while") {
            return Err(anyhow!("Error parsing while statement: missing \"while\""));
        }

        let expression = Or::parse(lexer)?;
        let block = Block::parse(lexer)?;
        Ok(ast::While::new(expression, block))
    }
}

impl Parser for FunctionCall {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let name = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("(")) {
            return Err(anyhow!("Error parsing function call: missing \'(\'"));
        }
        lexer.pop_char();

        let mut arguments = Vec::new();
        while !lexer.peek_and(|s| s.starts_with(")")) {
            let argument = Or::parse(lexer)?;
            arguments.push(argument);

            if lexer.peek_and(|s| s == ",") {
                lexer.next_token();
            }
        }
        lexer.pop_char();

        Ok(ast::FunctionCall::new(name, arguments))
    }
}

impl Parser for Assignment {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let target = parse_identifier(lexer)?;
        if !lexer.peek_and(|s| s.starts_with("=")) {
            return Err(anyhow!("Error parsing assignment: missing \"=\""));
        }
        lexer.pop_char();

        let value = Or::parse(lexer)?;
        if !lexer.peek_and(|s| s.starts_with(";")) {
            return Err(anyhow!("Error parsing assignment: missing \";\""));
        }
        lexer.pop_char();

        Ok(ast::Assignment::new(target, value))
    }
}
