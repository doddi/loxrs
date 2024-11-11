#[derive(Debug)]
pub enum LoxError {
    InvalidToken { error: &'static str },
    InvalidStatement { error: String },
    UnexpectedEof,
    InterpreterExpression,
    InterpreterStatement,
}
