use crate::{
    ast,
    lexer::{Lexer, Token},
};
use anyhow::{Result, anyhow};

pub fn parse_parameter(lexer: &mut Lexer) -> Result<(Token, ast::Type)> {
    let ident = if let Some(token) = lexer.consume_if(|token| token.is_identifier()) {
        token
    } else {
        return Err(anyhow!(
            "Failed to parse function parameter: expected identifier"
        ));
    };

    if lexer.consume_if(|token| token == &Token::Colon).is_none() {
        return Err(anyhow!("Failed to parse function parameter: missing colon"));
    }

    let ident_type = if let Some(token) = lexer.consume_if(|token| token.is_identifier())
        && let Token::Identifier(ident_type) = token
    {
        match ident_type.as_str() {
            "i32" => ast::Type::Integer32,
            "b8" => ast::Type::Boolean8,
            _ => {
                return Err(anyhow!(
                    "Failed to parse function parameter: unexpected identifier type"
                ));
            }
        }
    } else {
        return Err(anyhow!(
            "Failed to parse function parameter: missing identifier type"
        ));
    };

    Ok((ident, ident_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_parameter() -> Result<()> {
        let source = "_x123: i32".to_string();
        let mut lexer = Lexer::new(source);

        let (token, token_type) = parse_parameter(&mut lexer)?;

        if let Token::Identifier(token) = token {
            assert_eq!(token, "_x123");
        } else {
            return Err(anyhow!("Error: token is not identifier"));
        }

        assert_eq!(token_type, ast::Type::Integer32);

        Ok(())
    }
}
