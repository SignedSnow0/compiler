use crate::{
    ast::{AstNode, rvalues::*},
    parser::{Factor, Identifier, Instruction, Parser},
    peek::StringUtils,
};
use anyhow::{Result, anyhow};

impl Parser for Factor {
    type TNext = Instruction;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)> {
        let mut string = string.trim_mut();
        if string.peek().is_some_and(|c| c == '(') {
            string.remove(0);
            let (left, remainder) = Self::TNext::parse(string)?;

            string = remainder.trim_mut();
            if string.peek().is_some_and(|c| c == ')') {
                string.remove(0);

                return Ok((left, string));
            }

            return Err(anyhow!("failed to parse expression, expected ')'"));
        } else if string.peek().is_some_and(|c| c.is_numeric()) {
            let mut value_str = String::default();
            while string.peek().is_some_and(|c| c.is_digit(10)) {
                value_str.push(string.remove(0));
            }

            if let Ok(value) = value_str.parse::<i32>() {
                return Ok((Box::new(Integer { value }), string));
            } else {
                Err(anyhow!("failed to parse integer"))
            }
        } else {
            Identifier::parse(string)
        }
    }
}
