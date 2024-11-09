use lexer::lexer::Lexer;
use parser::parser::parse;

mod lexer;
mod parser;
mod eval;


pub fn run(source: &str) {
    let mut lexer = Lexer::new();

    if let Ok(()) = lexer.tokenize(source) {
        let mut tokens = lexer.get();
        let expression = parse(&mut tokens);

        println!("{}", expression);
    }
}



