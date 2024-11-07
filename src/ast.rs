// expression     → literal
//                | unary
//                | binary
//                | grouping ;
// 
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

use std::fmt::Display;

use crate::tokens::{Token, Tokens};

//#[derive(Debug)]
//pub(crate) enum Ast<'a> {
//    Expression(Box<Expression<'a>>),
//    Literal(Literal<'a>),
//    Grouping(Box<Expression<'a>>),
//    Unary(Unary<'a>),
//    Binary(Binary<'a>),
//    Operator(Operator),
//}

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Literal(Literal<'a>),
    Unary(Unary<'a>),
    Binary(Box<Expression<'a>>, Operator, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
}

impl <'a>Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::Unary(unary) => write!(f, "{}", unary),
            Expression::Binary(lhs, op, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
            Expression::Grouping(expression) => write!(f, "{}", expression),
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
pub(crate) enum Grouping<'a> {
    Expression(Expression<'a>)
}

impl <'a>Display for Grouping<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Grouping::Expression(expression) => write!(f, "({})", expression),
        };

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Unary<'a> {
    Negate(Box<Expression<'a>>),
    Not(Box<Expression<'a>>)
}

impl <'a>Display for Unary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Unary::Negate(expression) => write!(f, "-({})", expression),
            Unary::Not(expression) => write!(f, "!({})", expression),
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
        };

        Ok(())
    }
}

pub fn parse<'a>(tokens: &'a mut Tokens) -> Expression<'a> {
    parse_internal(tokens)
}

pub fn parse_internal<'a>(tokens: &mut Tokens<'a>) -> Expression<'a> {
    let lhs: Expression<'a> = match tokens.peek() {
        Some(token) => match token {
            Token::Number(_) => parse_literal(tokens),
            Token::String(_) => parse_literal(tokens),

            //Token::LeftParen => self.parse_group(&mut token_iter),
            _ => unreachable!("Should not get here"),
        },
        None => todo!(),
    };

    // See what the operation is
    let op = match tokens.peek() {
        Some(op) => match op {

            Token::Plus => Operator::Plus, 
            Token::Minus=> Operator::Minus, 
            Token::Star => Operator::Mult, 
            Token::Slash => Operator::Divide, 
            Token::Equal => Operator::EqualTo,
            Token::BangEqual => Operator::NotEqualTo,
            Token::Less => Operator::LessThan,
            Token::LessEqual => Operator::LessEqualThan,
            Token::Greater => Operator::GreaterThan,
            Token::GreaterEqual => Operator::GreaterEqualThan,
            other => unreachable!("invalid operation {:?}", other),
        },
        None => return lhs,
    };
    tokens.consume();

    let rhs: Expression<'a> = parse_internal(tokens);

    //Expression::Literal(Literal::Number(1.0))

    Expression::Binary(
        Box::new(lhs),
        op,
        Box::new(rhs)
    )
}


fn parse_literal<'a>(tokens: &mut Tokens<'a>) -> Expression<'a>
{
    let token = tokens.next();
    match token.unwrap() {
        Token::Number(val) => Expression::Literal(Literal::Number(*val)),
        Token::String(val) => match *val {
            "True" => Expression::Literal(Literal::True),
            "False" => Expression::Literal(Literal::False),
            "Nil" => Expression::Literal(Literal::Nil),
            _ => Expression::Literal(Literal::String(*val)),
        }
        _ => unreachable!("Unable to parse a literal"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Plus, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(+ 1.1 2.1)")
    }

    #[test]
    fn test_minus() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Minus, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(- 1.1 2.1)")
    }

    #[test]
    fn test_multiply() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Star, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(* 1.1 2.1)")
    }

    #[test]
    fn test_divide() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Slash, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(/ 1.1 2.1)")
    }

    #[test]
    fn test_equal_to() {
        let mut tokens = Tokens::new(vec![Token::String("Hello"), Token::Equal, Token::String("World")]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(== Hello World)")
    }

    #[test]
    fn test_not_equal_to() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::BangEqual, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(!= 1.1 2.1)")
    }

    #[test]
    fn test_less_than() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Less, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(< 1.1 2.1)")
    }

    #[test]
    fn test_less_than_equal() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::LessEqual, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(<= 1.1 2.1)")
    }

    #[test]
    fn test_greater_than() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Greater, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(> 1.1 2.1)")
    }

    #[test]
    fn test_greater_than_equal() {
        let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::GreaterEqual, Token::Number(2.1)]);
        let ast = parse(&mut tokens);

        assert_eq!(ast.to_string(), "(>= 1.1 2.1)")
    }

    //#[test]
    //fn test_negate() {
    //    let mut tokens = Tokens::new(vec![Token::Number(1.1), Token::Plus, Token::Minus, Token::Number(2.1)]);
    //    let ast = parse(&mut tokens);
    //
    //    assert_eq!(ast.to_string(), "(+ 1.1 -(2.1))")
    //}
}
