use std::usize;

use tracing::trace;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub(super) enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or twu charater tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    Error,
    Eof,
}

impl Iterator for TokenType {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TokenType::LeftParen => Some(TokenType::RightParen),
            TokenType::RightParen => Some(TokenType::LeftBrace),
            TokenType::LeftBrace => Some(TokenType::RightBrace),
            TokenType::RightBrace => Some(TokenType::Comma),
            TokenType::Comma => Some(TokenType::Dot),
            TokenType::Dot => Some(TokenType::Minus),
            TokenType::Minus => Some(TokenType::Plus),
            TokenType::Plus => Some(TokenType::SemiColon),
            TokenType::SemiColon => Some(TokenType::Slash),
            TokenType::Slash => Some(TokenType::Star),
            TokenType::Star => Some(TokenType::Bang),
            TokenType::Bang => Some(TokenType::BangEqual),
            TokenType::BangEqual => Some(TokenType::Equal),
            TokenType::Equal => Some(TokenType::EqualEqual),
            TokenType::EqualEqual => Some(TokenType::Greater),
            TokenType::Greater => Some(TokenType::GreaterEqual),
            TokenType::GreaterEqual => Some(TokenType::Less),
            TokenType::Less => Some(TokenType::LessEqual),
            TokenType::LessEqual => Some(TokenType::Identifier),
            TokenType::Identifier => Some(TokenType::String),
            TokenType::String => Some(TokenType::Number),
            TokenType::Number => Some(TokenType::And),
            TokenType::And => Some(TokenType::Class),
            TokenType::Class => Some(TokenType::Else),
            TokenType::Else => Some(TokenType::False),
            TokenType::False => Some(TokenType::For),
            TokenType::For => Some(TokenType::Fun),
            TokenType::Fun => Some(TokenType::If),
            TokenType::If => Some(TokenType::Nil),
            TokenType::Nil => Some(TokenType::Or),
            TokenType::Or => Some(TokenType::Print),
            TokenType::Print => Some(TokenType::Return),
            TokenType::Return => Some(TokenType::Super),
            TokenType::Super => Some(TokenType::This),
            TokenType::This => Some(TokenType::True),
            TokenType::True => Some(TokenType::Var),
            TokenType::Var => Some(TokenType::While),
            TokenType::While => Some(TokenType::Error),
            TokenType::Error => Some(TokenType::Eof),
            TokenType::Eof => None,
        }
    }
}

type StringId = usize;

#[derive(Debug, Copy, Clone)]
pub(super) struct Token {
    pub(super) token_type: TokenType,
    pub(super) id: StringId,
    pub(super) line: usize,
}

impl Token {
    pub(super) fn new(token_type: TokenType, id: StringId, line: usize) -> Self {
        trace!("Token::new({:?}, {}, {}", token_type, id, line);
        Self {
            token_type,
            id,
            line,
        }
    }
}
