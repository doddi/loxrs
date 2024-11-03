use core::panic;

use crate::token::{ Token, FloatWrapper };


fn tokenize(content: &'static str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
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
                    Ok(value) => tokens.push(Token::Number(FloatWrapper::Real(value))),
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
    Ok(tokens)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_eof() {
        let content = "";

        let response = tokenize(content).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response.get(0).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_parenthesis() {
        let content = "()";
 
        let response = tokenize(content).unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response.get(0).unwrap(), &Token::LeftParen);
        assert_eq!(response.get(1).unwrap(), &Token::RightParen);
        assert_eq!(response.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_brace() {
        let content = "{}";
 
        let response = tokenize(content).unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(response.get(1).unwrap(), &Token::RightBrace);
        assert_eq!(response.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_remainig_constants() {
        let content = ",.-+;/*";
 
        let response = tokenize(content).unwrap();
        assert_eq!(response.len(), 8);
        assert_eq!(response.get(0).unwrap(), &Token::Comma);
        assert_eq!(response.get(1).unwrap(), &Token::Dot);
        assert_eq!(response.get(2).unwrap(), &Token::Minus);
        assert_eq!(response.get(3).unwrap(), &Token::Plus);
        assert_eq!(response.get(4).unwrap(), &Token::Semicolon);
        assert_eq!(response.get(5).unwrap(), &Token::Slash);
        assert_eq!(response.get(6).unwrap(), &Token::Star);
        assert_eq!(response.get(7).unwrap(), &Token::Eof);
    }
    
    #[test]
    fn detect_bang() {
        let mut response = tokenize("!").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::Bang);

        response = tokenize("!=").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::BangEqual);
    }

    #[test]
    fn detect_equal() {
        let mut response = tokenize("=").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::Equal);

        response = tokenize("==").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::EqualEqual);
    }

    #[test]
    fn detect_greater() {
        let mut response = tokenize(">").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::Greater);

        response = tokenize(">=").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::GreaterEqual);
    }

    #[test]
    fn detect_less() {
        let mut response = tokenize("<").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::Less);

        response = tokenize("<=").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::LessEqual);
    }

    #[test]
    fn detect_comment() {
        let mut response = tokenize("/").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::Slash);

        response = tokenize("// This is a comment").unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response.get(0).unwrap(), &Token::Eof);

        let multi_line = r#"{
            // This is a comment
        }
        "#;
        let response = tokenize(multi_line).unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(response.get(1).unwrap(), &Token::RightBrace);
        assert_eq!(response.get(2).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_string() {
        let response = tokenize("\"Foo\"").unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response.get(0).unwrap(), &Token::String("Foo"));
        assert_eq!(response.get(1).unwrap(), &Token::Eof);

        let multi_line = r#"{
            "This is a string"
        }
        "#;
        let response = tokenize(multi_line).unwrap();
        assert_eq!(response.len(), 4);
        assert_eq!(response.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(response.get(1).unwrap(), &Token::String("This is a string"));
        assert_eq!(response.get(2).unwrap(), &Token::RightBrace);
        assert_eq!(response.get(3).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_number() {
        let mut response = tokenize("123.0").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Number(FloatWrapper::Real(123.0)));

        response = tokenize("456").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Number(FloatWrapper::Real(456.0)));

        assert!(tokenize("1.2.3").is_err_and(|err| err.contains("unable to parse number")));
    }

    #[test]
    fn detect_keywords() {
        let mut response;
        response = tokenize("and").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::And);

        response = tokenize("class").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Class);

        response = tokenize("else").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Else);

        response = tokenize("false").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::False);

        response = tokenize("fun").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Fun);

        response = tokenize("for").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::For);

        response = tokenize("if").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::If);

        response = tokenize("nil").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Nil);

        response = tokenize("or").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Or);

        response = tokenize("print").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Print);

        response = tokenize("return").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Return);

        response = tokenize("super").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Super);

        response = tokenize("this").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::This);

        response = tokenize("true").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::True);

        response = tokenize("var").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Var);

        response = tokenize("while").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::While);
    }

    #[test]
    fn detect_identifier() {
        let mut response;
        response = tokenize("hello").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Identifier("hello"));

        response = tokenize("_hello").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Identifier("_hello"));

        response = tokenize("_hel_lo_").unwrap();
        assert_eq!(response.get(0).unwrap(), &Token::Identifier("_hel_lo_"));
    }

    #[test]
    fn detect_handful_of_tokens() {
        let multi_line = r#"
        {
            // This is the start
            var myVar = print("statement");
        }
        "#;
        let response = tokenize(multi_line).unwrap();
        assert_eq!(response.len(), 11);
        assert_eq!(response.get(0).unwrap(), &Token::LeftBrace);
        assert_eq!(response.get(1).unwrap(), &Token::Var);
        assert_eq!(response.get(2).unwrap(), &Token::Identifier("myVar"));
        assert_eq!(response.get(3).unwrap(), &Token::Equal);
        assert_eq!(response.get(4).unwrap(), &Token::Print);
        assert_eq!(response.get(5).unwrap(), &Token::LeftParen);
        assert_eq!(response.get(6).unwrap(), &Token::String("statement"));
        assert_eq!(response.get(7).unwrap(), &Token::RightParen);
        assert_eq!(response.get(8).unwrap(), &Token::Semicolon);
        assert_eq!(response.get(9).unwrap(), &Token::RightBrace);
        assert_eq!(response.get(10).unwrap(), &Token::Eof);
    }

    #[test]
    fn detect_invalid_token() {
        let multi_line = r#"
            //Some comment
            $
            //Other comment
        "#;
        let response = tokenize(multi_line);
        assert!(response.is_err_and(|value| value.contains("Unrecognised token '$' at line: 2")));
    }
}
