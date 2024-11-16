use super::{loxerror::LoxError, string_indexer::StringId};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(StringId),
    String(StringId),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub(crate) struct TokenStore {
    inner: Vec<Token>,
    index: usize,
}

impl TokenStore {
    pub fn new(inner: Vec<Token>) -> Self {
        Self { inner, index: 0 }
    }

    pub fn consume(&mut self) {
        self.next();
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.inner.len() == self.index {
            return None;
        }
        let token = self.inner.get(self.index);
        self.index += 1;
        Some(token)?
    }

    pub fn peek(&self) -> Option<&Token> {
        self.inner.get(self.index)
    }

    pub fn expect(&self, expected: Token) -> Result<(), LoxError> {
        match self.inner.get(self.index) {
            Some(token) => {
                if token != &expected {
                    // TODO: Look to surface token information
                    return Err(LoxError::InvalidToken {
                        error: "Received unexpected token",
                    });
                }
                Ok(())
            }
            None => Err(LoxError::UnexpectedEof),
        }
    }

    pub(crate) fn is(&self, expect: Token) -> bool {
        match self.peek() {
            Some(token) => token == &expect,
            None => false,
        }
    }
}
