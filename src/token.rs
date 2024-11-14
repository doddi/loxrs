use crate::loxerror::LoxError;


#[derive(Debug, PartialEq)]
pub enum Token<'src> {
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

    Identifier(&'src str),
    String(&'src str),
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

pub struct Tokens<'src> {
    inner: Vec<Token<'src>>,
    index: usize,
}

impl <'src>Tokens<'src> {

    pub fn new(inner: Vec<Token<'src>>) -> Self {
       Self {
            inner, index: 0
        } 
    }

    pub fn consume(&mut self) {
        self.next();
    }

    pub fn next(&mut self) -> Option<&Token<'src>> {
        if self.inner.len() == self.index {
            return None
        }
        let token = self.inner.get(self.index);
        self.index += 1;
        Some(token)?
    }

    pub fn peek(&self) -> Option<&Token<'src>> {
        self.inner.get(self.index)
    }

    pub fn expect(&self, expected: Token<'src>) -> Result<(), LoxError> {
        match self.inner.get(self.index) {
            Some(token) => {
                if token != &expected {
                    // TODO: Look to surface token information
                    return Err(LoxError::InvalidToken { error: "Received unexpected token" });
                }
                Ok(())
            },
            None => Err(LoxError::UnexpectedEof),
        }
    }

    pub(crate) fn is(&self, expect: Token<'src>) -> bool {
        match self.peek() {
            Some(token) => token == &expect,
            None => false,
        }
    }
}
