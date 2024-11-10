use tracing::trace;

use crate::lexer::tokens::{ Tokens, Token };

use core::panic;

use super::{Expr, Literal, Operator};

/// This implementation is making use of the Pratt Parser technique

pub(crate) fn parse<'a>(tokens: &mut Tokens<'a>) -> Expr<'a> {
    parse_expression_binding_power(tokens, 0)
}


fn parse_literal<'a>(token: &Token<'a>) -> Expr<'a>
{
    trace!("parse_literal: {token:?}");

    match token {
        Token::Number(val) => Expr::Literal(Literal::Number(*val)),
        Token::String(val) => match *val {
            "true" => Expr::Literal(Literal::True),
            "false" => Expr::Literal(Literal::False),
            "Nil" => Expr::Literal(Literal::Nil),
            _ => Expr::Literal(Literal::String(val)),
        }
        _ => unreachable!("Unable to parse a literal"),
    }
}


fn parse_expression_binding_power<'a>(tokens: &mut Tokens<'a>, min_binding_power: u8) -> Expr<'a> {
    trace!("parse_expr_bp: {min_binding_power}");

    let mut lhs = match tokens.next() {
        Some(token) => match token {
            Token::Number(_) => parse_literal(token),
            Token::String(_) => parse_literal(token),

            Token::Minus => parse_unary(tokens, Operator::Negate),
            Token::Bang => parse_unary(tokens, Operator::Not),
            Token::LeftParen => parse_grouping(tokens),

            Token::True => Expr::Literal(Literal::True),
            Token::False => Expr::Literal(Literal::False),
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

        lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
    }
    lhs
}

fn parse_grouping<'a>(tokens: &mut Tokens<'a>) -> Expr<'a> {
    trace!("parse_grouping");

    let expression = parse_expression_binding_power(tokens, 0);
    tokens.consume();
    Expr::Grouping(Box::new(expression))
}

fn parse_unary<'a>(tokens: &mut Tokens<'a>, op: Operator) -> Expr<'a> {
    trace!("parse_unary operator: {op}");

    let min_binding_power = prefix_binding_power(&op).1;
    Expr::Unary(op, Box::new(parse_expression_binding_power(tokens, min_binding_power)))
}

fn parse_operator(token: &Token<'_>) -> Option<Operator> {
    trace!("parse_operator: {token:?}");

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
    trace!("infix_bp: {op}");
    
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
    trace!("prefix_bp: {op}");

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
