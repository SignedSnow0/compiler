use crate::{
    ast::{
        AstNode, BinaryAstNode,
        lvalues::Assignment,
        rvalues::{Integer, RValue},
    },
    parser::{
        Declaration, Identifier, Or, Parser,
        utils::{Type, parse_type},
    },
    peek::StringUtils,
};
use anyhow::Result;

impl Parser for Declaration {
    type TNext = Identifier;

    fn parse(string: String) -> Result<(Box<dyn AstNode>, String)> {
        let mut string = string.trim_mut();
        if !string.peek().is_some_and(|c| c == 'l') {
            return Err(anyhow::anyhow!(
                "failed to parse declaration, expected 'let'"
            ));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 'e') {
            return Err(anyhow::anyhow!(
                "failed to parse declaration, expected 'let'"
            ));
        }
        string.remove(0);
        if !string.peek().is_some_and(|c| c == 't') {
            return Err(anyhow::anyhow!(
                "failed to parse declaration, expected 'let'"
            ));
        }
        string.remove(0);

        let (identifier, mut remainder) = Self::TNext::parse(string)?;
        if !remainder.peek().is_some_and(|c| c == ':') {
            return Err(anyhow::anyhow!("failed to parse declaration, expected ':'"));
        }
        remainder.remove(0);

        let (value, mut remainder) = match parse_type(remainder.trim_mut())? {
            (Type::I32, remainder) => {
                let mut string = remainder.trim_mut();
                if string.peek().is_some_and(|c| c == '=') {
                    string.remove(0);
                    let (value, remaining) = Or::parse(string.trim_mut())?;
                    string = remaining.trim_mut();
                    (value, string)
                } else {
                    (Integer::default(), string)
                }
            }
        };

        match remainder.peek() {
            Some(';') => {
                remainder.remove(0);
                Ok((Assignment::new(identifier, value), remainder))
            }
            _ => Err(anyhow::anyhow!("failed to parse declaration, expected ';'")),
        }
    }
}

impl Parser for Identifier {
    type TNext = Or;

    fn parse(string: String) -> Result<(Box<dyn crate::ast::AstNode>, String)> {
        let mut string = string.trim_mut();
        let mut ident_str = String::default();

        if !string.peek().is_some_and(|c| c.is_alphabetic() || c == '_') {
            return Err(anyhow::anyhow!("failed to parse identifier"));
        }

        while string
            .peek()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            ident_str.push(string.remove(0));
        }

        Ok((
            Box::new(crate::ast::lvalues::Identifier { name: ident_str }),
            string,
        ))
    }
}
