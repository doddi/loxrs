use crate::{lexer::tokens::{Token, Tokens}, parser::{parser::parse, Expr}};

pub(crate) enum Statement<'a> {
    Print(Box<Expr<'a>>),
}

pub(crate) struct Parser<'a> {
    tokens: Tokens<'a>,
}

fn matches<'a>(lhs: &Token<'a>, rhs: &Token<'a>) -> bool {
   lhs == rhs
}

impl <'a>Parser<'a> {
    pub(crate) fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens,
        }
    }

    pub(crate) fn run(&mut self, tokens: Tokens<'a>, statements: Vec<Statement<'a>>) {
        loop {
            let token = tokens.peek();
            match token {
                Some(lhs) => {
                    if matches(lhs, &Token::Eof) {
                        statements.push(self.declaration());
                    }
                },
                None => break,
            }
        }
    }


    fn declaration(&'a mut self) -> Statement<'a> {
        self.statement().expect("expected a declaration")
    }

    fn statement(&'a mut self) -> Option<Statement<'a>> {
        match self.tokens.peek().unwrap() {
            Token::Print => {
                self.tokens.consume();
                return Some(self.print_statement())
            }
            _ => None,
        }
    }

    fn print_statement(&mut self) -> Statement {
        let value: Expr = self.expression();
        Statement::Print(Box::new(value))
    }

    fn expression(&mut self) -> Expr {
        parse(&mut self.tokens)
    }
}


