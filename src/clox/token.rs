use std::usize;

#[derive(Debug, PartialEq)]
pub(super) enum TokenType {
    // Single character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    SemiColon, Slash, Star,

    // One or twu charater tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error,
    Eof,
}

type StringId = usize;

pub(super) struct Token {
    pub(super) token_type: TokenType,
    id: StringId,
    pub(super) line: usize,
}

impl Token {
    pub(super) fn new(token_type: TokenType, id: StringId, line: usize) -> Self {
        Self {
            token_type,
            id,
            line,
        }
    }
}
