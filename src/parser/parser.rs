use crate::lexer::tokens::{ Tokens, Token };

use core::panic;
use std::fmt::Display;

/// This implementation is making use of the Pratt Parser technique

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Literal(Literal<'a>),
    Unary(Operator, Box<Expression<'a>>),
    Binary(Box<Expression<'a>>, Operator, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
}

impl <'a>Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::Unary(op, rhs) => write!(f, "{}{}", op, rhs),
            Expression::Binary(lhs, op, rhs) => write!(f, "{} {} {}", op, lhs, rhs),
            Expression::Grouping(expression) => write!(f, "({})", expression),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Literal<'a> {
    Number(f64),
    String(&'a str),
    True,
    False,
    Nil,
}

impl <'a>Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Literal::Number(val) => write!(f, "{}", val),
            Literal::String(val) => write!(f, "{}", val),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Operator {
    EqualTo,
    NotEqualTo,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    Plus,
    Minus,
    Mult,
    Divide,

    // TODO: Are these operators?
    Negate,
    Not,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Operator::EqualTo => write!(f, "=="),
            Operator::NotEqualTo => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessEqualThan => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterEqualThan => write!(f, ">="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Divide => write!(f, "/"),

            Operator::Negate => write!(f, "-"),
            Operator::Not => write!(f, "!"),
        };

        Ok(())
    }
}

fn parse_literal<'a>(token: &Token<'a>) -> Expression<'a>
{
    match token {
        Token::Number(val) => Expression::Literal(Literal::Number(*val)),
        Token::String(val) => match *val {
            "True" => Expression::Literal(Literal::True),
            "False" => Expression::Literal(Literal::False),
            "Nil" => Expression::Literal(Literal::Nil),
            _ => Expression::Literal(Literal::String(val)),
        }
        _ => unreachable!("Unable to parse a literal"),
    }
}


pub(crate) fn parse<'a>(tokens: &mut Tokens<'a>) -> Expression<'a> {
    parse_expression_binding_power(tokens, 0)
}

fn parse_expression_binding_power<'a>(tokens: &mut Tokens<'a>, min_binding_power: u8) -> Expression<'a> {
    let mut lhs = match tokens.next() {
        Some(token) => match token {
            Token::Number(_) => parse_literal(token),
            Token::String(_) => parse_literal(token),

            Token::Minus => parse_unary(tokens, Operator::Negate),
            Token::Bang => parse_unary(tokens, Operator::Not),
            Token::LeftParen => parse_grouping(tokens),
            t => unreachable!("Should not get here, invalid token: {:?}", t),
        },
        None => panic!("No token found"),
    };

    loop {
        let op = match tokens.peek() {
            Some(token) => match parse_operator(token) {
                Some(op) => op,
                None => break,
            },
            None => panic!("Expected a token operator"),
        };

        let (l_bind_power, r_bind_power) = infix_binding_power(&op);
        if l_bind_power < min_binding_power {
            break;
        }
        tokens.consume();

        let rhs = parse_expression_binding_power(tokens, r_bind_power);

        lhs = Expression::Binary(Box::new(lhs), op, Box::new(rhs));
    }
    lhs
}

fn parse_grouping<'a>(tokens: &mut Tokens<'a>) -> Expression<'a> {
    let expression = parse_expression_binding_power(tokens, 0);
    tokens.consume();
    Expression::Grouping(Box::new(expression))
}

fn parse_unary<'a>(tokens: &mut Tokens<'a>, op: Operator) -> Expression<'a> {
    let min_binding_power = prefix_binding_power(&op).1;
    Expression::Unary(op, Box::new(parse_expression_binding_power(tokens, min_binding_power)))
}

fn parse_operator(token: &Token<'_>) -> Option<Operator> {
    match token {
        Token::Eof => None,
        Token::Plus => Some(Operator::Plus),
        Token::Minus => Some(Operator::Minus),
        Token::Slash => Some(Operator::Divide),
        Token::Star => Some(Operator::Mult),

        Token::EqualEqual => Some(Operator::EqualTo),
        Token::BangEqual => Some(Operator::NotEqualTo),

        Token::Greater => Some(Operator::GreaterThan),
        Token::GreaterEqual => Some(Operator::GreaterEqualThan),
        Token::Less => Some(Operator::LessThan),
        Token::LessEqual => Some(Operator::LessEqualThan),

        _t => None,
    }
}



fn infix_binding_power(op: &Operator) -> (u8, u8) {
    match op {
        Operator::EqualTo | Operator::NotEqualTo => (1, 2),

        Operator::LessThan | Operator::LessEqualThan | 
        Operator::GreaterThan | Operator::GreaterEqualThan => (3, 4),

        Operator::Plus | Operator::Minus => (5, 6),
        Operator::Mult |Operator::Divide => (7, 8),
        _ => panic!("invalid infix operator: {:?}", op),
    }
}


fn prefix_binding_power(op: &Operator) -> ((), u8) {
    match op {
        Operator::Negate | Operator::Not => ((), 9),
        _ => panic!("invalid prefix operator: {:?}", op),
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;

    use super::*;

    fn lex<'a>(value: &'a str) -> String {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(value);
        parse(&mut lexer.get()).to_string()
    }

    #[test]
    fn test_add() {
        assert_eq!(lex("1.1 + 2.1"), "+ 1.1 2.1");
        assert_eq!(lex("1.1 - 2.1"), "- 1.1 2.1");
        assert_eq!(lex("1.1 * 2.1"), "* 1.1 2.1");
        assert_eq!(lex("1.1 / 2.1"), "/ 1.1 2.1");
    }


    #[test]
    fn test_equality() {
        assert_eq!(lex("\"Hello\" == \"World\""), "== \"Hello\" \"World\"");
        assert_eq!(lex("1.1 != 2.1"), "!= 1.1 2.1");
        assert_eq!(lex("1.1 < 2.1"), "< 1.1 2.1");
        assert_eq!(lex("1.1 <= 2.1"), "<= 1.1 2.1");
        assert_eq!(lex("1.1 > 2.1"), "> 1.1 2.1");
        assert_eq!(lex("1.1 >= 2.1"), ">= 1.1 2.1");
    }


    #[test]
    fn test_precedence() {
        assert_eq!(lex("1 + 2 * 3"), "+ 1 * 2 3");
    }

    #[test]
    fn test_unary() {
        assert_eq!(lex("-3"), "-3");
        assert_eq!(lex("!3"), "!3");
    }

    #[test]
    fn test_grouping() {
        assert_eq!(lex("(1 + 2) * 3"), "* (+ 1 2) 3");
    }
}
