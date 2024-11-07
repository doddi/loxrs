#[derive(Debug, PartialEq)]
pub enum Token<'a> {
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

    Identifier(&'a str),
    String(&'a str),
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

pub(crate) struct Tokens<'a> {
    inner: Vec<Token<'a>>,
    index: usize,
}

impl <'a>Tokens<'a> {

    pub fn new(inner: Vec<Token<'a>>) -> Self {
       Self {
            inner, index: 0
        } 
    }

    pub fn consume(&mut self) {
        self.next();
    }

    pub fn next(&mut self) -> Option<&Token<'a>> {
        if self.inner.len() == self.index {
            return None
        }
        let token = self.inner.get(self.index);
        self.index += 1;
        Some(token)?
    }

    pub fn peek(&self) -> Option<&Token<'a>> {
        self.inner.get(self.index)
    }
}
