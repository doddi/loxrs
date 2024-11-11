use tracing::trace;

use crate::{expr::Expr, expr_parser::ExprParser, loxerror::LoxError, statement::Statement, token::{Token, Tokens}};


pub(crate) struct StatementParser<'a> {
    tokens: Tokens<'a>,
    expr_parser: ExprParser,
}

impl <'a>StatementParser<'a> {
    pub(crate) fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens,
            expr_parser: ExprParser::new(),
        }
    }

    pub(crate) fn run(&mut self) -> Result<Vec<Statement<'a>>, LoxError> {
        let mut statements = vec![];

        loop {
            let token = self.tokens.peek();
            trace!("{:?}", token);
            match token {
                Some(lhs) => {
                    if lhs !=  &Token::Eof {
                        statements.push(self.parse_declaration()?);
                    }
                    else {
                        break;
                    }
                },
                None => break,
            }
        }
        Ok(statements)
    }


    fn parse_declaration(&mut self) -> Result<Statement<'a>, LoxError> {
        let declaration = match self.tokens.peek() {
            Some(token) => match token {
                Token::Var => todo!(),
                Token::Fun => todo!(),
                Token::Class => todo!(),
                _ => self.parse_statement(),
            },
            None => Err(LoxError::UnexpectedEof),
        };
        trace!("declarataion: {:?}", declaration);
        declaration
    }

    fn parse_statement(&mut self) -> Result<Statement<'a>, LoxError> {
        trace!("parse_statement");
        match self.tokens.peek() {
            Some(token) => match token {
                Token::Print => self.print_statement(),
                Token::If => self.if_statement(),
                Token::LeftBrace => {
                    self.tokens.consume();
                    return Ok(Statement::Block(self.block()?));
                }
                Token::While => todo!(),
                Token::Return => todo!(),
                Token::For => todo!(),
                _ => self.parse_expression_statement(),
            }
            None => Err(LoxError::UnexpectedEof),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement<'a>, LoxError> {
        let expr = self.expr_parser.parse(&mut self.tokens);
        Ok(Statement::Expression(Box::new(expr)))
    }

    fn print_statement(&mut self) -> Result<Statement<'a>, LoxError> {
        self.tokens.expect(Token::Print)?;
        self.tokens.consume();

        let value: Expr = self.expr_parser.parse(&mut self.tokens);

        self.tokens.expect(Token::Semicolon)?;
        self.tokens.consume();

        Ok(Statement::Print(Box::new(value)))
    }

    fn if_statement(&mut self) -> Result<Statement<'a>, LoxError> {
        self.tokens.expect(Token::If)?;
        self.tokens.consume();
        self.tokens.expect(Token::LeftParen)?;
        self.tokens.consume();

        let condition = self.expr_parser.parse(&mut self.tokens);

        self.tokens.expect(Token::RightParen)?;
        self.tokens.consume();

        let if_branch = self.parse_statement()?;
        let else_branch = match self.tokens.peek() {
            Some(token) => {
                if token == &Token::Else {
                    self.tokens.consume();
                    Some(self.parse_statement()?)
                }
                else {
                    None
                }
            },
            None => None,
        };

        Ok(Statement::If(Box::new(condition), Box::new(if_branch), else_branch.map(Box::new)))
    }

    fn block(&mut self) -> Result<Vec<Statement<'a>>, LoxError> {
        trace!("block entered");
        let mut statements = Vec::new();

        while !self.tokens.is(Token::RightBrace) {
            statements.push(self.parse_declaration()?);
        }

        let _ = self.tokens.expect(Token::RightBrace);
        self.tokens.consume();

        trace!("block exit, statements: {:?}", statements);
        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Lexer;

    use super::*;

    fn setup<'a>(source: &'a str) -> Vec<Statement<'a>> {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(source);
        let mut parser = StatementParser::new(lexer.get());
        //parser.run()
        todo!()
    }

    #[test]
    fn test() {
        let response = setup("print \"hello\"");
        assert_eq!(format!("{:?}", response), "");
    }
}
