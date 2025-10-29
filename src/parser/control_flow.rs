use anyhow::{Result, anyhow};

use crate::{
    ast::{
        AstNode, GenericAstNode,
        control_flow::{Block as AstBlock, If as AstIf, While as AstWhile},
    },
    parser::{Block, Declaration, Expression, If, Instruction, Or, Parser, While},
    peek::StringUtils,
};

impl Parser for Instruction {
    type TNext = Or;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)> {
        let string = string.trim_mut();

        match string.peek() {
            Some('l') => Declaration::parse(string),
            Some('{') => Block::parse(string),
            Some('i') => If::parse(string),
            Some('w') => While::parse(string),
            _ => Or::parse(string),
        }
    }
}

impl Parser for Block {
    type TNext = If;

    fn parse(string: String) -> anyhow::Result<(Box<dyn AstNode>, String)> {
        let mut string = string.trim_mut();

        if !string.peek().is_some_and(|c| c == '{') {
            return Err(anyhow!("failed parsing block, expected \'{}\'", '{'));
        }
        string.remove(0);
        let mut block = AstBlock::new();
        let mut string = string.trim_mut();

        while !string.peek().is_none_or(|c| c == '}') {
            let (instruction, remainder) = Instruction::parse(string.trim_mut())?;
            block.add_node(instruction);
            string = remainder.trim_mut();
        }
        if !string.peek().is_some_and(|c| c == '}') {
            return Err(anyhow!("failed parsing block, expected \'{}\'", '}'));
        }
        string.remove(0);

        Ok((block, string))
    }
}

impl Parser for If {
    type TNext = While;

    fn parse(string: String) -> anyhow::Result<(Box<dyn AstNode>, String)> {
        let mut string = string.trim_mut();

        if !string.peek().is_some_and(|c| c == 'i') {
            return Err(anyhow!("failed parsing block, expected \'if\'"));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'f') {
            return Err(anyhow!("failed parsing block, expected \'if\'"));
        }
        string.remove(0);

        let (expression, remainder) = Or::parse(string.trim_mut())?;
        let (then, remainder) = Block::parse(remainder.trim_mut())?;
        let mut string = remainder.trim_mut();
        let (otherwise, remainder) = if string.starts_with("else") {
            string.remove(0);
            string.remove(0);
            string.remove(0);
            string.remove(0);

            let (otherwise, remainder) = Block::parse(string.trim_mut())?;
            (Some(otherwise), remainder)
        } else {
            (None, string)
        };

        let node = AstIf::new(expression, then, otherwise);
        Ok((node, remainder))
    }
}

impl Parser for While {
    type TNext = Block;

    fn parse(string: String) -> anyhow::Result<(Box<dyn AstNode>, String)> {
        let mut string = string.trim_mut();

        if !string.peek().is_some_and(|c| c == 'w') {
            return Err(anyhow!("failed parsing block, expected \'while\'"));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'h') {
            return Err(anyhow!("failed parsing block, expected \'while\'"));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'i') {
            return Err(anyhow!("failed parsing block, expected \'while\'"));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'l') {
            return Err(anyhow!("failed parsing block, expected \'while\'"));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'e') {
            return Err(anyhow!("failed parsing block, expected \'while\'"));
        }
        string.remove(0);

        let (expression, remainder) = Or::parse(string.trim_mut())?;
        let (block, remainder) = Block::parse(remainder.trim_mut())?;
        let node = AstWhile::new(expression, block);
        Ok((node, remainder))
    }
}
