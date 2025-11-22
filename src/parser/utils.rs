use crate::Lexer;
use anyhow::{Result, anyhow};

pub fn parse_identifier(lexer: &mut Lexer) -> Result<String> {
    if let Some(token) = lexer.next_while(|c| c.is_alphanumeric() || c == '_') {
        if !(token.chars().nth(0).unwrap().is_alphabetic() || token.chars().nth(0).unwrap() == '_')
        {
            return Err(anyhow!(
                "Failed to parse identifier: must start with an alphanumeric character"
            ));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        let test_string = "variable_name123 ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));

        let identifier = parse_identifier(&mut lexer).unwrap();
        assert_eq!(identifier, "variable_name123");

        let test_string = "123invalid_name ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));
        let result = parse_identifier(&mut lexer);
        assert!(result.is_err());

        let test_string = "!@#invalid ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));
        let result = parse_identifier(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_parameter() {
        let test_string = "variable_name123: i32 ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));

        let (name, type_name) = parse_parameter(&mut lexer).unwrap();
        assert_eq!(name, "variable_name123");
        assert_eq!(type_name, "i32");

        let test_string = "123invalid: i32 ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));
        let result = parse_parameter(&mut lexer);
        assert!(result.is_err());

        let test_string = "variable_name123: 123invalid ";
        let reader = std::io::BufReader::new(std::io::Cursor::new(test_string));
        let mut lexer = Lexer::new(Box::new(reader));
        let result = parse_parameter(&mut lexer);
        assert!(result.is_err());
    }
}
