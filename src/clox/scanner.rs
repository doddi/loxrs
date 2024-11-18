use tracing::trace;

use super::{
    clox_error::CloxError,
    string_indexer::StringIndexer,
    token::{Token, TokenType},
};

pub(super) struct Scanner<'src> {
    content: &'src str,
    start: usize,
    current: usize,
    line: usize,

    string_indexer: StringIndexer<'src>,
}

impl<'src> Scanner<'src> {
    pub(super) fn new(content: &'src str) -> Self {
        Self {
            content,
            start: 0,
            current: 0,
            line: 0,

            string_indexer: StringIndexer::new(content),
        }
    }

    pub(super) fn scan_token(&mut self) -> Result<Token, CloxError> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.make_token(TokenType::Eof));
        }

        let c = self.advance();

        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            ';' => Ok(self.make_token(TokenType::SemiColon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '!' => {
                let result = if self.matches('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                };
                Ok(result)
            }
            '=' => {
                let result = if self.matches('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                };
                Ok(result)
            }
            '<' => {
                let result = if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                };
                Ok(result)
            }
            '>' => {
                let result = if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                };
                Ok(result)
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => Err(CloxError::UnexpectedToken),
        }
    }

    fn identifire_type(&self) -> Result<TokenType, CloxError> {
        match self.get_start_offset_char(0) {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lass", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.get_start_offset_char(1) {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => Ok(TokenType::Identifier),
                    }
                } else {
                    Ok(TokenType::Identifier)
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.get_start_offset_char(1) {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => Ok(TokenType::Identifier),
                    }
                } else {
                    Ok(TokenType::Identifier)
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => Ok(TokenType::Identifier),
        }
    }

    fn get_start_offset_char(&self, offset: usize) -> char {
        let pos = self.start + offset;
        let s = &self.content[pos..=pos];
        let check = s.to_string().chars().nth(0).expect("expect another char");
        check
    }

    fn matches(&mut self, expect: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expect {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.content.len() - 1
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let start = self.start;
        let end = self.current;
        let id = self.string_indexer.add_string(start, end);
        Token::new(token_type, id, self.line)
    }

    fn advance(&mut self) -> char {
        let ret = self.content[self.current..].chars().next().unwrap_or('\0');
        self.current += 1;
        ret
    }

    fn skip_whitespace(&mut self) {
        loop {
            let value = self.peek();
            match value {
                ' ' | '\r' | '\t' => {
                    let _ = self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> char {
        self.content[self.current..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let next = self.current + 1;
        self.content[next..].chars().next().unwrap_or('\0')
    }

    fn string(&mut self) -> Result<Token, CloxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(CloxError::UnterminatedString { line: self.line });
        }

        self.advance();
        Ok(self.make_token(TokenType::String))
    }

    fn number(&mut self) -> Result<Token, CloxError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        Ok(self.make_token(TokenType::Number))
    }

    fn identifier(&mut self) -> Result<Token, CloxError> {
        while self.is_alpha(self.peek()) || self.peek().is_ascii_digit() {
            self.advance();
        }

        Ok(self.make_token(self.identifire_type()?))
    }

    fn is_alpha(&self, value: char) -> bool {
        value.is_ascii_alphabetic() || value == '_'
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        remaining: &str,
        token_type: TokenType,
    ) -> Result<TokenType, CloxError> {
        let from = self.start + start;
        let to = self.start + start + length;

        let slice = &self.content[from..to];

        if (self.current - self.start == start + length) && (slice == remaining) {
            return Ok(token_type);
        }
        Ok(TokenType::Identifier)
    }

    pub(crate) fn get_str_at(&self, string_id: usize) -> &str {
        self.string_indexer.get_str_at(string_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_keyword() {
        let content = "true ";
        let mut scanner = Scanner::new(content);
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::True);
    }
}
