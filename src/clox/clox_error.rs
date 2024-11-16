#[derive(Debug)]
pub enum CloxError {
    CompileError,
    RuntimeError,
    StackUnderflow,
    UnexpectedToken,
    StringIndexOutOfBouds,
    UnterminatedString { line: usize },
}

