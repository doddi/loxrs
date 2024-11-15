use interpreter::Interpreter;
use loxerror::LoxError;
use statement_parser::StatementParser;
use string_indexer::StringIndexer;
use tokenizer::Lexer;

mod expr;
mod expr_parser;
mod function;
mod interpreter;
mod loxerror;
mod object;
mod statement;
mod statement_parser;
mod string_indexer;
mod token;
mod tokenizer;
mod environment;

pub fn run(source: &str) -> Result<(), LoxError> {
    let mut lexer = Lexer::new();

    let mut string_indexer = StringIndexer::new(source);
    if let Ok(mut token_store) = lexer.tokenize(&mut string_indexer, source) {
        let mut statement_parser = StatementParser::new();
        let statements = statement_parser.run(&mut token_store, &string_indexer)?;

        let mut interpreter = Interpreter::new();
        let _ = interpreter.run(&statements);
    }

    Ok(())
}
