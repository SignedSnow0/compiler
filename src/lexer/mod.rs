pub struct Lexer {
    source: Vec<char>,
    pub current_line: usize,
    pub current_pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Number(String),
    ParenL,
    ParenR,
    CurlyL,
    CurlyR,
    Semicolon,
    Colon,
    Comma,

    Plus,
    Minus,
    Multiplication,
    Division,
    Mod,
    Equal,

    Let,
    Fn,
    Struct,
    If,
    Else,
    While,
    For,
    Return,

    LogicalEqual,
    NotEqual,
    LogicalNot,
    LogicalOr,
    LogicalAnd,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,

    Other,
    EOF,
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        let dummy = Self::Identifier(String::new());
        std::mem::discriminant(self) == std::mem::discriminant(&dummy)
    }

    pub fn is_number(&self) -> bool {
        let dummy = Self::Number(String::new());
        std::mem::discriminant(self) == std::mem::discriminant(&dummy)
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let source = source.chars().rev().collect::<Vec<char>>();

        Self {
            source,
            current_line: 0,
            current_pos: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        let token = self.peek();
        match &token {
            Token::Identifier(value) | Token::Number(value) => {
                let start_index = self.source.len() - value.len();

                self.source.drain(start_index..);
                self.current_pos += value.len();
            }
            Token::LogicalEqual
            | Token::NotEqual
            | Token::LesserEqual
            | Token::GreaterEqual
            | Token::LogicalOr
            | Token::LogicalAnd
            | Token::If
            | Token::Fn => {
                let start_index = self.source.len() - 2;
                self.source.drain(start_index..);
                self.current_pos += 2;
            }
            Token::For | Token::Let => {
                let start_index = self.source.len() - 3;
                self.source.drain(start_index..);
                self.current_pos += 3;
            }
            Token::Else => {
                let start_index = self.source.len() - 4;
                self.source.drain(start_index..);
                self.current_pos += 4;
            }
            Token::While => {
                let start_index = self.source.len() - 5;
                self.source.drain(start_index..);
                self.current_pos += 5;
            }
            Token::Struct | Token::Return => {
                let start_index = self.source.len() - 6;
                self.source.drain(start_index..);
                self.current_pos += 6;
            }
            Token::EOF => {}
            _ => {
                self.source.pop();
                self.current_pos += 1;
            }
        };

        token
    }

    pub fn peek(&mut self) -> Token {
        while self.source.last().is_some_and(|c| c.is_whitespace()) {
            match self.source.pop() {
                Some(item) => {
                    if item == '\n' {
                        self.current_line += 1;
                        self.current_pos = 0;
                    } else if item == ' ' {
                        self.current_pos += 1;
                    }
                }
                None => {}
            }
        }
        let character = if let Some(c) = self.source.last() {
            c
        } else {
            return Token::EOF;
        };

        match character {
            '(' => Token::ParenL,
            ')' => Token::ParenR,
            '{' => Token::CurlyL,
            '}' => Token::CurlyR,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiplication,
            '/' => Token::Division,
            '%' => Token::Mod,
            ',' => Token::Comma,
            '=' => {
                if self
                    .source
                    .get(self.source.len() - 2)
                    .is_some_and(|c| *c == '=')
                {
                    Token::LogicalEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if self
                    .source
                    .get(self.source.len() - 2)
                    .is_some_and(|c| *c == '=')
                {
                    Token::NotEqual
                } else {
                    Token::LogicalNot
                }
            }
            '<' => {
                if self
                    .source
                    .get(self.source.len() - 2)
                    .is_some_and(|c| *c == '=')
                {
                    Token::LesserEqual
                } else {
                    Token::Lesser
                }
            }
            '>' => {
                if self
                    .source
                    .get(self.source.len() - 2)
                    .is_some_and(|c| *c == '=')
                {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            c if c.is_numeric() => Token::Number(self.parse_number()),
            _ => match self.parse_word().as_str() {
                "let" => Token::Let,
                "fn" => Token::Fn,
                "struct" => Token::Struct,
                "||" => Token::LogicalOr,
                "&&" => Token::LogicalAnd,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "for" => Token::For,
                "return" => Token::Return,
                _ => {
                    let ident = self.parse_identifier();
                    if !ident.is_empty() {
                        Token::Identifier(ident)
                    } else {
                        Token::Other
                    }
                }
            },
        }
    }

    pub fn peek_and(&mut self, predicate: fn(Token) -> bool) -> bool {
        let token = self.peek();

        predicate(token)
    }

    pub fn consume_if(&mut self, predicate: fn(Token) -> bool) -> Option<Token> {
        if self.peek_and(predicate) {
            Some(self.next())
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> String {
        let mut value = String::new();
        let mut index = self.source.len() - 1;
        while self
            .source
            .get(index)
            .is_some_and(|c| c.is_alphanumeric() || *c == '_')
        {
            value.insert(value.len(), *self.source.get(index).unwrap());
            if index == 0 {
                return value;
            }

            index -= 1;
        }

        value
    }

    fn parse_number(&mut self) -> String {
        let mut value = String::new();
        let mut index = self.source.len() - 1;

        let mut dot_parsed = false;
        while self
            .source
            .get(index)
            .is_some_and(|c| c.is_numeric() || (*c == '.' && !dot_parsed))
        {
            let new_char = *self.source.get(index).unwrap();
            value.insert(value.len(), new_char);
            if index == 0 {
                return value;
            }
            if new_char == '.' {
                dot_parsed = true;
            }

            index -= 1;
        }

        value
    }

    fn parse_word(&mut self) -> String {
        let mut value = String::new();
        let mut index = self.source.len() - 1;
        while self.source.get(index).is_some_and(|c| !c.is_whitespace()) {
            value.insert(value.len(), *self.source.get(index).unwrap());
            if index == 0 {
                return value;
            }

            index -= 1;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "let x: i32 = 10;\n".to_string();
        let mut lexer = Lexer::new(source);

        let expected = Token::Let;
        assert_eq!(lexer.next(), expected);

        let expected = Token::Identifier("x".to_string());
        assert_eq!(lexer.next(), expected);

        let expected = Token::Colon;
        assert_eq!(lexer.next(), expected);

        let expected = Token::Identifier("i32".to_string());
        assert_eq!(lexer.next(), expected);

        let expected = Token::Equal;
        assert_eq!(lexer.next(), expected);

        let expected = Token::Number("10".to_string());
        assert_eq!(lexer.next(), expected);

        let expected = Token::Semicolon;
        assert_eq!(lexer.next(), expected);

        assert_eq!(lexer.current_line, 0);
        assert_eq!(lexer.current_pos, 16);

        let expected = Token::EOF;
        assert_eq!(lexer.next(), expected);

        assert_eq!(lexer.current_line, 1);
        assert_eq!(lexer.current_pos, 0);
    }
}
