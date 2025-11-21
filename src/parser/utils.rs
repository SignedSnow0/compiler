use crate::Lexer;
use anyhow::{Result, anyhow};

pub fn parse_identifier(lexer: &mut Lexer) -> Result<String> {
    if let Some(token) = lexer.next_while(|c| c.is_alphanumeric() || c == '_') {
        Ok(token)
    } else {
        Err(anyhow!(
            "Failed to parse identifier: expected alphanumeric characters or '_'"
        ))
    }
}

pub fn parse_parameter(lexer: &mut Lexer) -> Result<(String, String)> {
    let name = parse_identifier(lexer)?;
    if !lexer.next_token().is_some_and(|s| s == ":") {
        return Err(anyhow!("Error parsing parameter: missing \":\""));
    }
    let type_name = parse_identifier(lexer)?;

    Ok((name, type_name))
}
