use eval::evaluate;
use lexer::lexer::Lexer;
use parser::parser::parse;

mod lexer;
mod parser;
mod eval;
mod statement;


pub fn run(source: &str) {
    let mut lexer = Lexer::new();

    if let Ok(()) = lexer.tokenize(source) {
        let mut tokens = lexer.get();
        let expression = parse(&mut tokens);
        let eval = evaluate(expression);

        if let Ok(result) = eval {
            println!("{:?}", result);
        }
    }
}


#[derive(Debug)]
pub enum LoxError {
    InvalidToken { error: &'static str },
}
