use tracing::trace;

use crate::{
    expr::Expr,
    expr_parser::ExprParser,
    loxerror::LoxError,
    statement::Statement,
    string_indexer::StringIndexer,
    token::{Token, TokenStore},
};

pub(crate) struct StatementParser {
    expr_parser: ExprParser,
}

impl StatementParser {
    pub(crate) fn new() -> Self {
        Self {
            expr_parser: ExprParser::new(),
        }
    }

    pub(crate) fn run(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Vec<Statement>, LoxError> {
        let mut statements = vec![];

        loop {
            let token = token_store.peek();
            trace!("{:?}", token);
            match token {
                Some(lhs) => {
                    if lhs != &Token::Eof {
                        statements.push(self.parse_declaration(token_store, string_indexer)?);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        Ok(statements)
    }

    fn parse_declaration(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Statement, LoxError> {
        let declaration = match token_store.peek() {
            Some(token) => match token {
                Token::Var => todo!(),
                Token::Fun => self.function(token_store, string_indexer, "function"),
                Token::Class => todo!(),
                _ => self.parse_statement(token_store, string_indexer),
            },
            None => Err(LoxError::UnexpectedEof),
        };
        trace!("declarataion: {:?}", declaration);
        declaration
    }

    fn parse_statement(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Statement, LoxError> {
        trace!("parse_statement");
        match token_store.peek() {
            Some(token) => match token {
                Token::Print => self.print_statement(token_store, string_indexer),
                Token::If => self.if_statement(token_store, string_indexer),
                Token::LeftBrace => {
                    token_store.consume();
                    return Ok(Statement::Block(self.block(token_store, string_indexer)?));
                }
                Token::While => todo!(),
                Token::Return => todo!(),
                Token::For => todo!(),
                _ => self.parse_expression_statement(token_store, string_indexer),
            },
            None => Err(LoxError::UnexpectedEof),
        }
    }

    fn parse_expression_statement(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Statement, LoxError> {
        let expr = self.expr_parser.parse(token_store, string_indexer);
        Ok(Statement::Expression(Box::new(expr)))
    }

    fn print_statement(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Statement, LoxError> {
        token_store.expect(Token::Print)?;
        token_store.consume();

        let value: Expr = self.expr_parser.parse(token_store, string_indexer);

        token_store.expect(Token::Semicolon)?;
        token_store.consume();

        Ok(Statement::Print(Box::new(value)))
    }

    fn if_statement(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Statement, LoxError> {
        token_store.expect(Token::If)?;
        token_store.consume();
        token_store.expect(Token::LeftParen)?;
        token_store.consume();

        let condition = self.expr_parser.parse(token_store, string_indexer);

        token_store.expect(Token::RightParen)?;
        token_store.consume();

        let if_branch = self.parse_statement(token_store, string_indexer)?;
        let else_branch = match token_store.peek() {
            Some(token) => {
                if token == &Token::Else {
                    token_store.consume();
                    Some(self.parse_statement(token_store, string_indexer)?)
                } else {
                    None
                }
            }
            None => None,
        };

        Ok(Statement::If(
            Box::new(condition),
            Box::new(if_branch),
            else_branch.map(Box::new),
        ))
    }

    fn block(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
    ) -> Result<Vec<Statement>, LoxError> {
        trace!("block entered");
        let mut statements = Vec::new();

        while !token_store.is(Token::RightBrace) {
            statements.push(self.parse_declaration(token_store, string_indexer)?);
        }

        let _ = token_store.expect(Token::RightBrace);
        token_store.consume();

        trace!("block exit, statements: {:?}", statements);
        Ok(statements)
    }

    fn function(
        &mut self,
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer,
        _arg: &'static str,
    ) -> Result<Statement, LoxError> {
        let name = token_store.next().expect("Expected a name token").clone();

        let _ = token_store.expect(Token::LeftParen);
        token_store.consume();

        let mut args: Vec<Token> = vec![];
        while !token_store.is(Token::RightParen) {
            if args.len() > 255 {
                return Err(LoxError::InvalidToken {
                    error: "Too many arguments to function call",
                });
            }

            match token_store.next() {
                Some(token) => match token {
                    Token::Identifier(_) => args.push(token.clone()),
                    t => trace!("Expected an identifier, found {:?}", t),
                },
                None => return Err(LoxError::UnexpectedEof),
            }
        }
        let _ = token_store.expect(Token::RightParen);
        token_store.consume();

        let _ = token_store.expect(Token::LeftBrace);
        token_store.consume();

        let body = self.block(token_store, string_indexer)?;

        Ok(Statement::Function { name, args, body })
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Lexer;

    use super::*;

    fn setup(source: &str) -> Vec<Statement> {
        let mut lexer = Lexer::new();
        let mut string_indexer = StringIndexer::new(source);
        let _ = lexer.tokenize(&mut string_indexer, source);
        //let mut parser = StatementParser::new(lexer.get());
        //parser.run()
        todo!()
    }

    #[test]
    fn test() {
        let response = setup("print \"hello\"");
        assert_eq!(format!("{:?}", response), "");
    }
}
