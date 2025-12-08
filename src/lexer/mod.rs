pub struct Lexer {
    pub tokens: Vec<Token>,
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

    BitAnd,
    BitOr,

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
        let tokens = Self::parse_tokens(source);

        Self {
            tokens,
            current_line: 0,
            current_pos: 0,
        }
    }

    pub fn peek(&self) -> &Token {
        self.tokens.last().unwrap_or(&Token::EOF)
    }

    pub fn peek_n(&self, n: usize) -> &Token {
        self.tokens
            .iter()
            .nth(self.tokens.len() - n)
            .unwrap_or(&Token::EOF)
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EOF)
    }

    pub fn peek_and(&self, predicate: fn(&Token) -> bool) -> bool {
        let token = self.peek();

        predicate(token)
    }

    pub fn peek_and_n(&self, n: usize, predicate: fn(&Token) -> bool) -> bool {
        let token = self.peek_n(n);

        predicate(token)
    }

    pub fn consume_if(&mut self, predicate: fn(&Token) -> bool) -> Option<Token> {
        if self.peek_and(predicate) {
            Some(self.next())
        } else {
            None
        }
    }

    fn parse_tokens(source: Vec<char>) -> Vec<Token> {
        let mut source = source;
        let mut tokens = Vec::new();

        let mut token = Self::parse_token(&mut source);
        while token != Token::EOF {
            tokens.insert(0, token);
            token = Self::parse_token(&mut source);
        }

        tokens
    }

    fn parse_token(source: &mut Vec<char>) -> Token {
        while source.last().is_some_and(|c| c.is_whitespace()) {
            source.pop();
        }

        let character = match source.pop() {
            Some(c) => c,
            None => return Token::EOF,
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
                if source.last().is_some_and(|c| *c == '=') {
                    source.pop();
                    Token::LogicalEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if source.last().is_some_and(|c| *c == '=') {
                    source.pop();
                    Token::NotEqual
                } else {
                    Token::LogicalNot
                }
            }
            '<' => {
                if source.last().is_some_and(|c| *c == '=') {
                    source.pop();
                    Token::LesserEqual
                } else {
                    Token::Lesser
                }
            }
            '>' => {
                if source.last().is_some_and(|c| *c == '=') {
                    source.pop();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if source.last().is_some_and(|c| *c == '&') {
                    source.pop();
                    Token::LogicalAnd
                } else {
                    Token::BitAnd
                }
            }
            '|' => {
                if source.last().is_some_and(|c| *c == '|') {
                    source.pop();
                    Token::LogicalOr
                } else {
                    Token::BitOr
                }
            }
            c if c.is_numeric() => Token::Number(Self::parse_number(source, c)),
            c if c.is_alphanumeric() || c == '_' => {
                let word = Self::parse_word(source, character);
                match word.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    "struct" => Token::Struct,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "return" => Token::Return,
                    _ => Token::Identifier(word),
                }
            }
            _ => {
                eprintln!("Error parsing char: {character}");
                Token::Other
            }
        }
    }

    fn parse_word(source: &mut Vec<char>, first: char) -> String {
        let mut value = first.to_string();
        while source
            .last()
            .is_some_and(|c| c.is_alphanumeric() || *c == '_')
        {
            value.insert(value.len(), source.pop().unwrap());
        }

        value
    }

    fn parse_number(source: &mut Vec<char>, first: char) -> String {
        let mut value = first.to_string();
        let mut dot_parsed = false;
        while source
            .last()
            .is_some_and(|c| c.is_numeric() || (*c == '.' && !dot_parsed))
        {
            if *source.last().unwrap() == '.' {
                dot_parsed = true;
            }
            value.insert(value.len(), source.pop().unwrap());
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

        let expected = Token::EOF;
        assert_eq!(lexer.next(), expected);
    }
}
