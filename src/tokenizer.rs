use tracing::trace;

use crate::token::{Token, Tokens};

#[derive(Debug)]
pub struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn get(self) -> Tokens<'a> {
        Tokens::new(self.tokens)
    }

    pub fn tokenize(&mut self, content: &'a str) -> Result<(), String> {
        let mut line_no = 0;

        let mut char_iter = content.char_indices().peekable();

        while char_iter.peek().is_some() {
            let (pos, ch) = char_iter.next().unwrap();

            match ch {
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                '{' => self.tokens.push(Token::LeftBrace),
                '}' => self.tokens.push(Token::RightBrace),
                ',' => self.tokens.push(Token::Comma),
                '.' => self.tokens.push(Token::Dot),
                '-' => self.tokens.push(Token::Minus),
                '+' => self.tokens.push(Token::Plus),
                ';' => self.tokens.push(Token::Semicolon),
                '*' => self.tokens.push(Token::Star),
                '/' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '/' {
                            char_iter.next();
                            while char_iter.peek().is_some() && char_iter.peek().unwrap().1 != '\n'
                            {
                                char_iter.next();
                            }
                        } else {
                            self.tokens.push(Token::Slash);
                        }
                    }
                    None => self.tokens.push(Token::Slash),
                },

                '!' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            self.tokens.push(Token::BangEqual);
                            char_iter.next();
                        } else {
                            self.tokens.push(Token::Bang);
                        }
                    }
                    None => self.tokens.push(Token::Bang),
                },
                '=' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            self.tokens.push(Token::EqualEqual);
                            char_iter.next();
                        } else {
                            self.tokens.push(Token::Equal);
                        }
                    }
                    None => self.tokens.push(Token::Equal),
                },
                '>' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            self.tokens.push(Token::GreaterEqual);
                            char_iter.next();
                        } else {
                            self.tokens.push(Token::Greater);
                        }
                    }
                    None => self.tokens.push(Token::Greater),
                },
                '<' => match char_iter.peek() {
                    Some((_, next)) => {
                        if *next == '=' {
                            self.tokens.push(Token::LessEqual);
                            char_iter.next();
                        } else {
                            self.tokens.push(Token::Less);
                        }
                    }
                    None => self.tokens.push(Token::Less),
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
                    let slice = &content[start..=end];
                    trace!("{:?}", slice);
                    self.tokens.push(Token::String(slice));

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
                        Ok(value) => self.tokens.push(Token::Number(value)),
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
                        "and" => self.tokens.push(Token::And),
                        "class" => self.tokens.push(Token::Class),
                        "else" => self.tokens.push(Token::Else),
                        "false" => self.tokens.push(Token::False),
                        "fun" => self.tokens.push(Token::Fun),
                        "for" => self.tokens.push(Token::For),
                        "if" => self.tokens.push(Token::If),
                        "nil" => self.tokens.push(Token::Nil),
                        "or" => self.tokens.push(Token::Or),
                        "print" => self.tokens.push(Token::Print),
                        "return" => self.tokens.push(Token::Return),
                        "super" => self.tokens.push(Token::Super),
                        "this" => self.tokens.push(Token::This),
                        "true" => self.tokens.push(Token::True),
                        "var" => self.tokens.push(Token::Var),
                        "while" => self.tokens.push(Token::While),
                        _ => self.tokens.push(Token::Identifier(slice)),
                    }
                }

                ' ' | '\r' | '\t' => (),
                '\n' => line_no += 1,
                _ => return Err(format!("Unrecognised token '{}' at line: {}", ch, line_no)),
            }
        }

        self.tokens.push(Token::Eof);
        Ok(())
    }
}

impl <'a>IntoIterator for Lexer<'a> {
    type Item = Token<'a>;
    type IntoIter = <Vec<Token<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_eof() {
        let content = "";

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(content).unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_parenthesis() {
        let content = "()";

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(content).unwrap();
        let mut iter = lexer.into_iter();

        assert_eq!(iter.next().unwrap(), Token::LeftParen);
        assert_eq!(iter.next().unwrap(), Token::RightParen);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_brace() {
        let content = "{}";

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(content).unwrap();
        let mut iter = lexer.into_iter();

        assert_eq!(iter.next().unwrap(), Token::LeftBrace);
        assert_eq!(iter.next().unwrap(), Token::RightBrace);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_remainig_constants() {
        let content = ",.-+;/*";

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(content).unwrap();
        let mut iter = lexer.into_iter();
        
        assert_eq!(iter.len(), 8);
        assert_eq!(iter.next().unwrap(), Token::Comma);
        assert_eq!(iter.next().unwrap(), Token::Dot);
        assert_eq!(iter.next().unwrap(), Token::Minus);
        assert_eq!(iter.next().unwrap(), Token::Plus);
        assert_eq!(iter.next().unwrap(), Token::Semicolon);
        assert_eq!(iter.next().unwrap(), Token::Slash);
        assert_eq!(iter.next().unwrap(), Token::Star);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_bang() {
        let mut lexer = Lexer::new();
        _ = lexer.tokenize("!").unwrap();
        let mut iter = lexer.into_iter();

        assert_eq!(iter.next().unwrap(), Token::Bang);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("!=").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::BangEqual);
    }

    #[test]
    fn detect_equal() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("=").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Equal);

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("==").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::EqualEqual);
    }

    #[test]
    fn detect_greater() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(">").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Greater);

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(">=").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::GreaterEqual);
    }

    #[test]
    fn detect_less() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("<").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Less);

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("<=").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::LessEqual);
    }

    #[test]
    fn detect_comment() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("/").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Slash);

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("// This is a comment").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Eof);

        let multi_line = r#"{
            // This is a comment
        }
        "#;
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(multi_line).unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::LeftBrace);
        assert_eq!(iter.next().unwrap(), Token::RightBrace);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_string() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("\"Foo\"").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::String("\"Foo\""));
        assert_eq!(iter.next().unwrap(), Token::Eof);

        let multi_line = r#"{
            "This is a string"
        }
        "#;
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(multi_line).unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::LeftBrace);
        assert_eq!(iter.next().unwrap(), Token::String("\"This is a string\""));
        assert_eq!(iter.next().unwrap(), Token::RightBrace);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_number() {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("123.0").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Number(123.0));

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("456").unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::Number(456.0));

        let mut lexer = Lexer::new();
        assert!(
            lexer.tokenize("1.2.3").is_err_and(|err| err.contains("unable to parse number"))
        );
    }

    #[test]
    fn detect_keywords() {
        let mut lexer = Lexer::new();
        _ = lexer.tokenize("and").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::And);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("class").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Class);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("else").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Else);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("false").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::False);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("fun").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Fun);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("for").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::For);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("if").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::If);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("nil").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Nil);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("or").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Or);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("print").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Print);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("return").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Return);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("super").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Super);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("this").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::This);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("true").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::True);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("var").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Var);

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("while").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::While);
    }

    #[test]
    fn detect_identifier() {
        let mut lexer = Lexer::new();
        _ = lexer.tokenize("hello").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Identifier("hello"));

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("_hello").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Identifier("_hello"));

        let mut lexer = Lexer::new();
        _ = lexer.tokenize("_hel_lo_").unwrap();
        assert_eq!(lexer.into_iter().next().unwrap(), Token::Identifier("_hel_lo_"));
    }

    #[test]
    fn detect_handful_of_tokens() {
        let multi_line = r#"
        {
            // This is the start
            var myVar = print("statement");
        }
        "#;
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(multi_line).unwrap();
        let mut iter = lexer.into_iter();
        assert_eq!(iter.next().unwrap(), Token::LeftBrace);
        assert_eq!(iter.next().unwrap(), Token::Var);
        assert_eq!(iter.next().unwrap(), Token::Identifier("myVar"));
        assert_eq!(iter.next().unwrap(), Token::Equal);
        assert_eq!(iter.next().unwrap(), Token::Print);
        assert_eq!(iter.next().unwrap(), Token::LeftParen);
        assert_eq!(iter.next().unwrap(), Token::String("\"statement\""));
        assert_eq!(iter.next().unwrap(), Token::RightParen);
        assert_eq!(iter.next().unwrap(), Token::Semicolon);
        assert_eq!(iter.next().unwrap(), Token::RightBrace);
        assert_eq!(iter.next().unwrap(), Token::Eof);
    }

    #[test]
    fn detect_invalid_token() {

        let multi_line = r#"
            //Some comment
            $
            //Other comment
        "#;
        let mut lexer = Lexer::new();
        let response = lexer.tokenize(multi_line);
        assert!(response.is_err_and(|value| value.contains("Unrecognised token '$' at line: 2")));
    }
}
