use crate::{ast::AstNode, parser::Parser, peek::StringUtils};
use anyhow::Result;

pub enum Type {
    I32,
}

pub fn parse_binary_op<T, TNext, F>(
    string: String,
    separators: &[char],
    right_parse: F,
) -> Result<(Box<dyn AstNode>, String)>
where
    TNext: Parser,
    F: Fn(Box<dyn AstNode>, String) -> Result<(Box<dyn AstNode>, String)>,
{
    let (mut left, string) = TNext::parse(string.trim_mut())?;
    let mut string = string.trim_mut();

    while string.peek().is_some_and(|c| separators.contains(&c)) {
        let (parsed, remainder) = right_parse(left, string)?;
        left = parsed;
        string = remainder;
    }

    Ok((left, string))
}

pub fn parse_type(mut string: String) -> Result<(Type, String)> {
    if !string.peek().is_some_and(|c| c == 'i') {
        return Err(anyhow::anyhow!("failed to parse type"));
    }
    string.remove(0);
    if !string.peek().is_some_and(|c| c == '3') {
        return Err(anyhow::anyhow!("failed to parse type"));
    }
    string.remove(0);
    if !string.peek().is_some_and(|c| c == '2') {
        return Err(anyhow::anyhow!("failed to parse type"));
    }
    string.remove(0);
    Ok((Type::I32, string))
}
