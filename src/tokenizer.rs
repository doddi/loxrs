use crate::{
    string_indexer::StringIndexer,
    token::{Token, TokenStore},
};

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(
        &mut self,
        string_indexer: &mut StringIndexer,
        content: &str,
    ) -> Result<TokenStore, String> {
        let mut line_no = 0;

        let mut tokens = Vec::new();
        let mut char_iter = content.char_indices().peekable();

        while char_iter.peek().is_some() {
            let (pos, ch) = char_iter.next().unwrap();

            match ch {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                ',' => tokens.push(Token::Comma),
                '.' => tokens.push(Token::Dot),
                '-' => tokens.push(Token::Minus),
                '+' => tokens.push(Token::Plus),
                ';' => tokens.push(Token::Semicolon),
                '*' => tokens.push(Token::Star),
                '/' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '/' {
                            char_iter.next();
                            while char_iter.peek().is_some() && char_iter.peek().unwrap().1 != '\n'
                            {
                                char_iter.next();
                            }
                        } else {
                            tokens.push(Token::Slash);
                        }
                    }
                    None => tokens.push(Token::Slash),
                },

                '!' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::BangEqual);
                            char_iter.next();
                        } else {
                            tokens.push(Token::Bang);
                        }
                    }
                    None => tokens.push(Token::Bang),
                },
                '=' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::EqualEqual);
                            char_iter.next();
                        } else {
                            tokens.push(Token::Equal);
                        }
                    }
                    None => tokens.push(Token::Equal),
                },
                '>' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::GreaterEqual);
                            char_iter.next();
                        } else {
                            tokens.push(Token::Greater);
                        }
                    }
                    None => tokens.push(Token::Greater),
                },
                '<' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::LessEqual);
                            char_iter.next();
                        } else {
                            tokens.push(Token::Less);
                        }
                    }
                    None => tokens.push(Token::Less),
                },

                '"' => {
                    let start = pos;
                    let mut end = start;
                    while char_iter.peek().is_some_and(|(_, value)| *value != '"') {
                        let (i, _) = char_iter.next().unwrap();
                        end = i;
                    }
                    // Account for the end '"'
                    char_iter.next();

                    end += 1;
                    let string_id = string_indexer.add_string(start, end);
                    tokens.push(Token::String(string_id));
                }

                '0'..='9' => {
                    let start = pos;
                    let mut end = start;
                    while char_iter
                        .peek()
                        .is_some_and(|(_, value)| value.is_ascii_digit() || *value == '.')
                    {
                        let (i, _) = char_iter.next().unwrap();
                        end = i;
                    }

                    let slice = &content[start..=end];
                    match slice.parse() {
                        Ok(value) => tokens.push(Token::Number(value)),
                        Err(_) => return Err(format!("unable to parse number: {}", slice)),
                    }
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = pos;
                    let mut end = start;
                    while char_iter
                        .peek()
                        .is_some_and(|(_, value)| value.is_alphanumeric() || *value == '_')
                    {
                        let (i, _) = char_iter.next().unwrap();
                        end = i;
                    }

                    let slice = &content[start..=end];
                    match slice {
                        "and" => tokens.push(Token::And),
                        "class" => tokens.push(Token::Class),
                        "else" => tokens.push(Token::Else),
                        "false" => tokens.push(Token::False),
                        "fun" => tokens.push(Token::Fun),
                        "for" => tokens.push(Token::For),
                        "if" => tokens.push(Token::If),
                        "nil" => tokens.push(Token::Nil),
                        "or" => tokens.push(Token::Or),
                        "print" => tokens.push(Token::Print),
                        "return" => tokens.push(Token::Return),
                        "super" => tokens.push(Token::Super),
                        "this" => tokens.push(Token::This),
                        "true" => tokens.push(Token::True),
                        "var" => tokens.push(Token::Var),
                        "while" => tokens.push(Token::While),
                        _ => {
                            let string_id = string_indexer.add_string(start, end);
                            tokens.push(Token::Identifier(string_id))
                        }
                    }
                }

                ' ' | '\r' | '\t' => (),
                '\n' => line_no += 1,
                _ => return Err(format!("Unrecognised token '{}' at line: {}", ch, line_no)),
            }
        }

        tokens.push(Token::Eof);
        Ok(TokenStore::new(tokens))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::panic;

    fn setup(content: &str) -> TokenStore {
        let mut string_indexer = StringIndexer::new(content);
        setup_with_indexer(&mut string_indexer, content)
    }

    fn setup_with_indexer(string_indexer: &mut StringIndexer, content: &str) -> TokenStore {
        let mut lexer = Lexer::new();
        lexer.tokenize(string_indexer, content).unwrap()
    }

    #[test]
    fn detect_eof() {
        let content = "";
        let mut token_store = setup(content);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_parenthesis() {
        let content = "()";
        let mut token_store = setup(content);

        assert_eq!(*token_store.next().unwrap(), Token::LeftParen);
        assert_eq!(*token_store.next().unwrap(), Token::RightParen);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_brace() {
        let content = "{}";
        let mut token_store = setup(content);

        assert_eq!(*token_store.next().unwrap(), Token::LeftBrace);
        assert_eq!(*token_store.next().unwrap(), Token::RightBrace);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_remainig_constants() {
        let content = ",.-+;/*";
        let mut token_store = setup(content);

        assert_eq!(*token_store.next().unwrap(), Token::Comma);
        assert_eq!(*token_store.next().unwrap(), Token::Dot);
        assert_eq!(*token_store.next().unwrap(), Token::Minus);
        assert_eq!(*token_store.next().unwrap(), Token::Plus);
        assert_eq!(*token_store.next().unwrap(), Token::Semicolon);
        assert_eq!(*token_store.next().unwrap(), Token::Slash);
        assert_eq!(*token_store.next().unwrap(), Token::Star);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_bang() {
        let mut token_store = setup("!");
        assert_eq!(*token_store.next().unwrap(), Token::Bang);

        let mut token_store = setup("!=");
        assert_eq!(*token_store.next().unwrap(), Token::BangEqual);
    }

    #[test]
    fn detect_equal() {
        let mut token_store = setup("=");
        assert_eq!(*token_store.next().unwrap(), Token::Equal);

        let mut token_store = setup("==");
        assert_eq!(*token_store.next().unwrap(), Token::EqualEqual);
    }

    #[test]
    fn detect_greater() {
        let mut token_store = setup(">");
        assert_eq!(*token_store.next().unwrap(), Token::Greater);

        let mut token_store = setup(">=");
        assert_eq!(*token_store.next().unwrap(), Token::GreaterEqual);
    }

    #[test]
    fn detect_less() {
        let mut token_store = setup("<");
        assert_eq!(*token_store.next().unwrap(), Token::Less);

        let mut token_store = setup("<=");
        assert_eq!(*token_store.next().unwrap(), Token::LessEqual);
    }

    #[test]
    fn detect_comment() {
        let mut token_store = setup("/");
        assert_eq!(*token_store.next().unwrap(), Token::Slash);

        let mut token_store = setup("// This is a comment");
        assert_eq!(*token_store.next().unwrap(), Token::Eof);

        let multi_line = r#"{
            // This is a comment
        }
        "#;
        let mut token_store = setup(multi_line);
        assert_eq!(*token_store.next().unwrap(), Token::LeftBrace);
        assert_eq!(*token_store.next().unwrap(), Token::RightBrace);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_string() {
        let content = "\"Foo\"";
        let mut string_indexer = StringIndexer::new(content);
        let mut token_store = setup_with_indexer(&mut string_indexer, content);
        next_token_string_is(&mut token_store, &string_indexer, content);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);

        let multi_line = r#"{
            "This is a string"
        }
        "#;
        let mut string_indexer = StringIndexer::new(content);
        let mut token_store = setup_with_indexer(&mut string_indexer, multi_line);
        assert_eq!(*token_store.next().unwrap(), Token::LeftBrace);
        next_token_string_is(&mut token_store, &string_indexer, "\"This is a string\"");
        assert_eq!(*token_store.next().unwrap(), Token::RightBrace);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_number() {
        let mut token_store = setup("123.0");
        assert_eq!(*token_store.next().unwrap(), Token::Number(123.0));

        let mut token_store = setup("456");
        assert_eq!(*token_store.next().unwrap(), Token::Number(456.0));

        let content = "1.2.3";
        let mut lexer = Lexer::new();
        let mut string_indexer = StringIndexer::new(content);
        assert!(lexer
            .tokenize(&mut string_indexer, content)
            .is_err_and(|err| err.contains("unable to parse number")));
    }

    #[test]
    fn detect_keywords() {
        let mut token_store = setup("and");
        assert_eq!(*token_store.next().unwrap(), Token::And);

        let mut token_store = setup("class");
        assert_eq!(*token_store.next().unwrap(), Token::Class);

        let mut token_store = setup("else");
        assert_eq!(*token_store.next().unwrap(), Token::Else);

        let mut token_store = setup("false");
        assert_eq!(*token_store.next().unwrap(), Token::False);

        let mut token_store = setup("fun");
        assert_eq!(*token_store.next().unwrap(), Token::Fun);

        let mut token_store = setup("for");
        assert_eq!(*token_store.next().unwrap(), Token::For);

        let mut token_store = setup("if");
        assert_eq!(*token_store.next().unwrap(), Token::If);

        let mut token_store = setup("nil");
        assert_eq!(*token_store.next().unwrap(), Token::Nil);

        let mut token_store = setup("or");
        assert_eq!(*token_store.next().unwrap(), Token::Or);

        let mut token_store = setup("print");
        assert_eq!(*token_store.next().unwrap(), Token::Print);

        let mut token_store = setup("return");
        assert_eq!(*token_store.next().unwrap(), Token::Return);

        let mut token_store = setup("super");
        assert_eq!(*token_store.next().unwrap(), Token::Super);

        let mut token_store = setup("this");
        assert_eq!(*token_store.next().unwrap(), Token::This);

        let mut token_store = setup("true");
        assert_eq!(*token_store.next().unwrap(), Token::True);

        let mut token_store = setup("var");
        assert_eq!(*token_store.next().unwrap(), Token::Var);

        let mut token_store = setup("while");
        assert_eq!(*token_store.next().unwrap(), Token::While);
    }

    #[test]
    fn detect_identifier() {
        assert_identifier_token("hello");
        assert_identifier_token("_hello");
        assert_identifier_token("_hel_lo");
    }

    fn assert_identifier_token(content: &'static str) {
        let mut string_indexer = StringIndexer::new(content);
        let mut token_store = setup_with_indexer(&mut string_indexer, content);

        next_token_identifier_is(&mut token_store, &string_indexer, content);
    }

    fn next_token_identifier_is(
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer<'_>,
        content: &str,
    ) {
        match token_store.next().unwrap() {
            Token::Identifier(string_id) => {
                assert_eq!(string_indexer.get_string(*string_id).unwrap(), content);
            }
            _ => panic!("error"),
        }
    }

    fn next_token_string_is(
        token_store: &mut TokenStore,
        string_indexer: &StringIndexer<'_>,
        content: &str,
    ) {
        match token_store.next().unwrap() {
            Token::String(string_id) => {
                assert_eq!(string_indexer.get_string(*string_id).unwrap(), content);
            }
            _ => panic!("error"),
        }
    }

    #[test]
    fn detect_handful_of_tokens() {
        let multi_line = r#"
        {
            // This is the start
            var myVar = print("statement");
        }
        "#;
        let mut string_indexer = StringIndexer::new(multi_line);
        let mut token_store = setup_with_indexer(&mut string_indexer, multi_line);
        assert_eq!(*token_store.next().unwrap(), Token::LeftBrace);
        assert_eq!(*token_store.next().unwrap(), Token::Var);
        next_token_identifier_is(&mut token_store, &string_indexer, "myVar");
        assert_eq!(*token_store.next().unwrap(), Token::Equal);
        assert_eq!(*token_store.next().unwrap(), Token::Print);
        assert_eq!(*token_store.next().unwrap(), Token::LeftParen);
        next_token_string_is(&mut token_store, &string_indexer, "\"statement\"");
        assert_eq!(*token_store.next().unwrap(), Token::RightParen);
        assert_eq!(*token_store.next().unwrap(), Token::Semicolon);
        assert_eq!(*token_store.next().unwrap(), Token::RightBrace);
        assert_eq!(*token_store.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_invalid_token() {
        let multi_line = r#"
            //Some comment
            $
            //Other comment
        "#;
        let mut lexer = Lexer::new();
        let mut string_indexer = StringIndexer::new(multi_line);
        let response = lexer.tokenize(&mut string_indexer, multi_line);
        assert!(response.is_err_and(|value| value.contains("Unrecognised token '$' at line: 2")));
    }
}
