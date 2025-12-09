use crate::{
    ast::{self, AstNode},
    lexer::{Lexer, Token},
    parser::{
        Assignment, Block, Declaration, FunctionCall, If, Instruction, Or, Parser, Return, While,
        utils::parse_char_lit,
    },
};
use anyhow::{Result, anyhow};

impl Parser for Block {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::CurlyL).is_none() {
            return Err(anyhow!("Failed to parse block: missing \"{{\""));
        }

        let mut block = ast::Block::new();
        while lexer.peek_and(|token| token != &Token::CurlyR) {
            block.add_node(Instruction::parse(lexer)?);
        }

        if lexer.consume_if(|token| token == &Token::CurlyR).is_none() {
            return Err(anyhow!("Failed to parse block: missing \"}}\""));
        }

        Ok(block)
    }
}

impl Parser for Instruction {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        match lexer.peek() {
            Token::Let => Declaration::parse(lexer),
            Token::If => If::parse(lexer),
            Token::While => While::parse(lexer),
            Token::Return => Return::parse(lexer),
            Token::Identifier(_) if lexer.peek_and_n(2, |token| token == &Token::ParenL) => {
                FunctionCall::parse(lexer)
            }
            Token::Identifier(_) if lexer.peek_and_n(2, |token| token == &Token::Equal) => {
                Assignment::parse(lexer)
            }
            _ => Err(anyhow!(
                "Failed to parse instruction: unexpected token \"{:?}\"",
                lexer.peek()
            )),
        }
    }
}

impl Parser for Return {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::Return).is_none() {
            return Err(anyhow!(
                "Error parsing return statement: missing \"return\""
            ));
        }

        let expression = Or::parse(lexer)?;
        if lexer
            .consume_if(|token| token == &Token::Semicolon)
            .is_none()
        {
            return Err(anyhow!("Error parsing return statement: missing \";\""));
        }

        Ok(ast::Return::new(expression))
    }
}

impl Parser for If {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::If).is_none() {
            return Err(anyhow!("Error parsing if statement: missing \"if\""));
        }

        let expression = Or::parse(lexer)?;
        let then_block = Block::parse(lexer)?;

        let else_block = if lexer.peek_and(|token| token == &Token::Else) {
            let _ = lexer.next();
            Some(Block::parse(lexer)?)
        } else {
            None
        };

        Ok(ast::If::new(expression, then_block, else_block))
    }
}

impl Parser for While {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        if lexer.consume_if(|token| token == &Token::While).is_none() {
            return Err(anyhow!("Error parsing while statement: missing \"while\""));
        }

        let expression = Or::parse(lexer)?;
        let block = Block::parse(lexer)?;
        Ok(ast::While::new(expression, block))
    }
}

impl Parser for FunctionCall {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let name = if let Some(token) = lexer.consume_if(|token| token.is_identifier())
            && let Token::Identifier(name) = token
        {
            name
        } else {
            return Err(anyhow!(
                "Error parsing function call: missing target identitifer"
            ));
        };

        if lexer.consume_if(|token| token == &Token::ParenL).is_none() {
            return Err(anyhow!("Error parsing function call: missing \'(\'"));
        }

        let mut arguments = Vec::new();
        while !lexer.peek_and(|token| token == &Token::ParenR) {
            let argument = Or::parse(lexer)?;
            arguments.push(argument);

            let _ = lexer.consume_if(|token| token == &Token::Comma);
        }

        if lexer.consume_if(|token| token == &Token::ParenR).is_none() {
            return Err(anyhow!("Error parsing function call: missing \')\'"));
        }

        Ok(ast::FunctionCall::new(name, arguments))
    }
}

impl Parser for Assignment {
    fn parse(lexer: &mut Lexer) -> Result<Box<dyn AstNode>> {
        let name = if let Some(token) = lexer.consume_if(|token| token.is_identifier())
            && let Token::Identifier(name) = token
        {
            name
        } else {
            return Err(anyhow!(
                "Error parsing assignment: missing target identitifer"
            ));
        };

        if lexer.consume_if(|token| token == &Token::Equal).is_none() {
            return Err(anyhow!("Error parsing assignment: missing \"=\""));
        }

        let value = Or::parse(lexer)?;

        if lexer
            .consume_if(|token| token == &Token::Semicolon)
            .is_none()
        {
            return Err(anyhow!("Error parsing assignment: missing \";\""));
        }

        Ok(ast::Assignment::new(name, value))
    }
}
