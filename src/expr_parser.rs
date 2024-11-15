use tracing::trace;

use core::panic;

use crate::{
    expr::{Expr, Literal, Operator},
    string_indexer::StringIndexer,
    token::{Token, TokenStore},
};

/// This implementation is making use of the Pratt Parser technique
pub(crate) struct ExprParser {}

impl ExprParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, token_store: &mut TokenStore, string_indexer: &StringIndexer) -> Expr {
        parse_expression_binding_power(token_store, string_indexer, 0)
    }
}

fn parse_expression_binding_power(
    token_store: &mut TokenStore,
    string_indexer: &StringIndexer,
    min_binding_power: u8,
) -> Expr {
    trace!("parse_expr_bp: {min_binding_power}");

    let mut lhs = match token_store.next() {
        Some(token) => match token {
            Token::Number(val) => Expr::Literal(Literal::Number(*val)),
            Token::String(string_id) => {
                trace!("string_id: {}", string_id);
                let value = string_indexer
                    .get_string(string_id.clone())
                    .expect("Should have value");
                return parse_string(value);
            }
            Token::Minus => parse_unary(token_store, string_indexer, Operator::Negate),
            Token::Bang => parse_unary(token_store, string_indexer, Operator::Not),
            Token::LeftParen => parse_grouping(token_store, string_indexer),

            Token::True => Expr::Literal(Literal::Bool(true)),
            Token::False => Expr::Literal(Literal::Bool(false)),
            t => unreachable!("Should not get here, invalid token: {:?}", t),
        },
        None => panic!("No token found"),
    };
    trace!("expression lhs: {lhs}");

    loop {
        let op = match token_store.peek() {
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
        token_store.consume();

        let rhs = parse_expression_binding_power(token_store, string_indexer, r_bind_power);

        lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
    }
    trace!("expression parse:{:?}", lhs);
    lhs
}

fn parse_string(value: &str) -> Expr {
    trace!("parse_string id: {value:?}");

    match value {
        "true" => Expr::Literal(Literal::Bool(true)),
        "false" => Expr::Literal(Literal::Bool(false)),
        "Nil" => Expr::Literal(Literal::Nil),
        _ => Expr::Literal(Literal::String(value.to_string())),
    }
}

fn parse_grouping(tokens: &mut TokenStore, string_indexer: &StringIndexer) -> Expr {
    trace!("parse_grouping");

    let expression = parse_expression_binding_power(tokens, string_indexer, 0);
    tokens.consume();
    Expr::Grouping(Box::new(expression))
}

fn parse_unary(tokens: &mut TokenStore, string_indexer: &StringIndexer, op: Operator) -> Expr {
    trace!("parse_unary operator: {op}");

    let min_binding_power = prefix_binding_power(&op).1;
    Expr::Unary(
        op,
        Box::new(parse_expression_binding_power(
            tokens,
            string_indexer,
            min_binding_power,
        )),
    )
}

fn parse_operator(token: &Token) -> Option<Operator> {
    let op = match token {
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
    };
    trace!("parse_operator: {op:?}");
    op
}

fn infix_binding_power(op: &Operator) -> (u8, u8) {
    trace!("infix_bp: {op}");

    match op {
        Operator::EqualTo | Operator::NotEqualTo => (1, 2),

        Operator::LessThan
        | Operator::LessEqualThan
        | Operator::GreaterThan
        | Operator::GreaterEqualThan => (3, 4),

        Operator::Plus | Operator::Minus => (5, 6),
        Operator::Mult | Operator::Divide => (7, 8),
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

    use crate::tokenizer::Lexer;

    use super::*;

    fn expr_parse_test<'a>(value: &'a str) -> String {
        let mut lexer = Lexer::new();
        let mut string_indexer = StringIndexer::new(value);
        let mut token_store = lexer.tokenize(&mut string_indexer, value).unwrap();
        let expr_parser = ExprParser::new();
        expr_parser
            .parse(&mut token_store, &string_indexer)
            .to_string()
    }

    #[test]
    fn test_add() {
        assert_eq!(expr_parse_test("1.1 + 2.1"), "+ 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 - 2.1"), "- 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 * 2.1"), "* 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 / 2.1"), "/ 1.1 2.1");
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            expr_parse_test("\"Hello\" == \"World\""),
            "== \"Hello\" \"World\""
        );
        assert_eq!(expr_parse_test("1.1 != 2.1"), "!= 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 < 2.1"), "< 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 <= 2.1"), "<= 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 > 2.1"), "> 1.1 2.1");
        assert_eq!(expr_parse_test("1.1 >= 2.1"), ">= 1.1 2.1");
    }

    #[test]
    fn test_precedence() {
        assert_eq!(expr_parse_test("1 + 2 * 3"), "+ 1 * 2 3");
    }

    #[test]
    fn test_unary() {
        assert_eq!(expr_parse_test("-3"), "-3");
        assert_eq!(expr_parse_test("!3"), "!3");
    }

    #[test]
    fn test_grouping() {
        assert_eq!(expr_parse_test("(1 + 2) * 3"), "* (+ 1 2) 3");
    }

    #[test]
    fn test_function() {
        let content = include_str!("../resources/function.lox");
        let parsed = expr_parse_test(content);
        assert_eq!(parsed, "");
    }
}
