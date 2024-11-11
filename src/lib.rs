use interpreter::Interpreter;
use loxerror::LoxError;
use statement_parser::StatementParser;
use tokenizer::Lexer;

mod tokenizer;
mod expr_parser;
mod expr;
mod statement;
mod token;
mod loxerror;
mod interpreter;
mod object;
mod statement_parser;


pub fn run(source: &str) -> Result<(), LoxError> {
    let mut lexer = Lexer::new();

    if let Ok(()) = lexer.tokenize(source) {
        let tokens = lexer.get();

        let mut statement_parser = StatementParser::new(tokens);
        let statements = statement_parser.run()?;

        let mut interpreter = Interpreter::new();
        let _ = interpreter.run(&statements);
    }

    Ok(())
}


