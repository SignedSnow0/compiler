use crate::{ast::AstNode, parser::Parser, peek::StringUtils};
use anyhow::Result;

pub fn parse_binary_op<T, TNext, F>(
    string: String,
    separators: &[char],
    right_parse: F,
) -> Result<(Box<dyn AstNode<TEval = T>>, String)>
where
    TNext: Parser<T>,
    F: Fn(Box<dyn AstNode<TEval = T>>, String) -> Result<(Box<dyn AstNode<TEval = T>>, String)>,
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
