use crate::token::Token;


pub fn tokenize<'a>(content: &'a str, tokens: &mut Vec<Token<'a>>) -> Result<(), String> {
    let mut line_no = 0;

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
            '/' => {
                match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '/' {
                            char_iter.next();
                            while char_iter.peek().is_some() && char_iter.peek().unwrap().1 != '\n' {
                                char_iter.next();
                            }
                        }
                        else {
                            tokens.push(Token::Slash);
                        }
                    },
                    None => tokens.push(Token::Slash),
                }
            }

            '!' => {
                match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::BangEqual);
                            char_iter.next();
                        }
                        else {
                            tokens.push(Token::Bang);
                        }
                    },
                    None => tokens.push(Token::Bang),
                }
            }
            '=' => {
                match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::EqualEqual);
                            char_iter.next();
                        }
                        else {
                            tokens.push(Token::Equal);
                        }
                    },
                    None => tokens.push(Token::Equal),
                }
            }
            '>' => {
                match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::GreaterEqual);
                            char_iter.next();
                        }
                        else {
                            tokens.push(Token::Greater);
                        }
                    },
                    None => tokens.push(Token::Greater),
                }
            }
            '<' => {
                match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            tokens.push(Token::LessEqual);
                            char_iter.next();
                        }
                        else {
                            tokens.push(Token::Less);
                        }
                    },
                    None => tokens.push(Token::Less),
                }
            }

            '"' => {
                let start = pos + 1;
                let mut end = start;
                while char_iter.peek().is_some_and(|(_, value)| *value != '"') {
                    let (i, _) = char_iter.next().unwrap();
                    end = i;
                }

                let slice = &content[start..=end];
                tokens.push(Token::String(slice));

                // Account for the end '"'
                char_iter.next();
            }

            '0'..='9' => {
                let start = pos;
                let mut end = start;
                while char_iter.peek().is_some_and(|(_, value)| value.is_digit(10) || *value == '.') {
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
                while char_iter.peek().is_some_and(|(_, value)| value.is_alphanumeric() || *value == '_'){
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
                    _ => tokens.push(Token::Identifier(slice)),
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => line_no += 1,
            _ => return Err(format!("Unrecognised token '{}' at line: {}", ch, line_no)),
        }
    }

    tokens.push(Token::Eof);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_eof() {
        let content = "";
        let mut tokens: Vec<Token<'_>> = Vec::new();

        let _ = tokenize(content, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.get(0).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_parenthesis() {
        let content = "()";
        let mut tokens: Vec<Token<'_>> = Vec::new();
 
        let _ = tokenize(content, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.get(0).unwrap(), &Token::LeftParen);
        assert_eq!(tokens.get(1).unwrap(), &Token::RightParen);
        assert_eq!(tokens.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_brace() {
        let content = "{}";
        let mut tokens: Vec<Token<'_>> = Vec::new();
 
        let _ = tokenize(content, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(tokens.get(1).unwrap(), &Token::RightBrace);
        assert_eq!(tokens.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_remainig_constants() {
        let content = ",.-+;/*";
        let mut tokens: Vec<Token<'_>> = Vec::new();
 
        let _ = tokenize(content, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens.get(0).unwrap(), &Token::Comma);
        assert_eq!(tokens.get(1).unwrap(), &Token::Dot);
        assert_eq!(tokens.get(2).unwrap(), &Token::Minus);
        assert_eq!(tokens.get(3).unwrap(), &Token::Plus);
        assert_eq!(tokens.get(4).unwrap(), &Token::Semicolon);
        assert_eq!(tokens.get(5).unwrap(), &Token::Slash);
        assert_eq!(tokens.get(6).unwrap(), &Token::Star);
        assert_eq!(tokens.get(7).unwrap(), &Token::Eof);
    }
    
    #[test]
    fn detect_bang() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize("!", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::Bang);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("!=", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::BangEqual);
    }

    #[test]
    fn detect_equal() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize("=", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::Equal);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("==", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::EqualEqual);
    }

    #[test]
    fn detect_greater() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize(">", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::Greater);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize(">=", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::GreaterEqual);
    }

    #[test]
    fn detect_less() {
        let mut tokens: Vec<Token<'_>> = Vec::new();

        let _ = tokenize("<", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::Less);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("<=", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::LessEqual);
    }

    #[test]
    fn detect_comment() {
        let mut tokens: Vec<Token<'_>> = Vec::new();

        let _ = tokenize("/", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::Slash);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("// This is a comment", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.get(0).unwrap(), &Token::Eof);

        let multi_line = r#"{
            // This is a comment
        }
        "#;
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize(multi_line, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(tokens.get(1).unwrap(), &Token::RightBrace);
        assert_eq!(tokens.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_string() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize("\"Foo\"", &mut tokens).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.get(0).unwrap(), &Token::String("Foo"));
        assert_eq!(tokens.get(1).unwrap(), &Token::Eof);

        let multi_line = r#"{
            "This is a string"
        }
        "#;
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize(multi_line, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(tokens.get(1).unwrap(), &Token::String("This is a string"));
        assert_eq!(tokens.get(2).unwrap(), &Token::RightBrace);
        assert_eq!(tokens.get(3).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_number() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        let _ = tokenize("123.0", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Number(123.0));

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("456", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Number(456.0));

        assert!(tokenize("1.2.3", &mut tokens).is_err_and(|err| err.contains("unable to parse number")));
    }

    #[test]
    fn detect_keywords() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("and", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::And);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("class", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Class);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("else", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Else);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("false", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::False);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("fun", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Fun);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("for", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::For);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("if", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::If);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("nil", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Nil);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("or", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Or);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("print", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Print);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("return", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Return);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("super", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Super);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("this", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::This);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("true", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::True);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("var", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Var);

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("while", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::While);
    }

    #[test]
    fn detect_identifier() {
        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("hello", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Identifier("hello"));

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("_hello", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Identifier("_hello"));

        let mut tokens: Vec<Token<'_>> = Vec::new();
        _ = tokenize("_hel_lo_", &mut tokens).unwrap();
        assert_eq!(tokens.get(0).unwrap(), &Token::Identifier("_hel_lo_"));
    }

    #[test]
    fn detect_handful_of_tokens() {
        let mut tokens: Vec<Token<'_>> = Vec::new();

        let multi_line = r#"
        {
            // This is the start
            var myVar = print("statement");
        }
        "#;
        let _ = tokenize(multi_line, &mut tokens).unwrap();
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(tokens.get(1).unwrap(), &Token::Var);
        assert_eq!(tokens.get(2).unwrap(), &Token::Identifier("myVar"));
        assert_eq!(tokens.get(3).unwrap(), &Token::Equal);
        assert_eq!(tokens.get(4).unwrap(), &Token::Print);
        assert_eq!(tokens.get(5).unwrap(), &Token::LeftParen);
        assert_eq!(tokens.get(6).unwrap(), &Token::String("statement"));
        assert_eq!(tokens.get(7).unwrap(), &Token::RightParen);
        assert_eq!(tokens.get(8).unwrap(), &Token::Semicolon);
        assert_eq!(tokens.get(9).unwrap(), &Token::RightBrace);
        assert_eq!(tokens.get(10).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_invalid_token() {
        let mut tokens: Vec<Token<'_>> = Vec::new();

        let multi_line = r#"
            //Some comment
            $
            //Other comment
        "#;
        let response = tokenize(multi_line, &mut tokens);
        assert!(response.is_err_and(|value| value.contains("Unrecognised token '$' at line: 2")));
    }
}
