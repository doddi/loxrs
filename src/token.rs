#[derive(Debug, PartialEq, Eq)]
pub enum Token {
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

    Identifier(&'static str),
    String(&'static str),
    Number(FloatWrapper),

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

#[derive(Debug)]
pub enum FloatWrapper {
    Real(f32),
    Nan,
}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FloatWrapper::Real(lhs), FloatWrapper::Real(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Eq for FloatWrapper {
    
}
