use std::fmt::Display;

use crate::{parser::{Expr, Literal, Operator}, LoxError};

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    // Value types
    Number(f64),
    String(&'a str),
    Bool(bool),
    Nil,

    // Unary
    Not(Box<Self>),
    Negate(Box<Self>),

    // Conditional
    Equality(Box<Self>, Equality, Box<Self>),

    // Operational
    Op(Box<Self>, Op, Box<Self>),
}

impl <'a>From<Box<Expression<'a>>> for Expression<'a> {
    fn from(value: Box<Expression<'a>>) -> Expression<'a> {
        *value
    }
}

#[derive(Debug)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub(crate) enum Equality {
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
}


impl <'a>Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{val}"),
            Self::String(str) => write!(f, "{}", *str),
            Self::Bool(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
            Self::Not(expression) => write!(f, "!{expression}"),
            Self::Negate(expression) => write!(f, "-{expression}"),
            Self::Equality(lhs, eq, rhs) => {
                let equality = match eq {
                    Equality::Eq => "==",
                    Equality::NotEq => "!=",
                    Equality::Lt => "<",
                    Equality::Lte => "<=",
                    Equality::Gt => ">",
                    Equality::Gte => ">=",
                };
                write!(f, "{lhs} {equality} {rhs}")
            },
            Self::Op(lhs, op, rhs) => {
                let operator = match op {
                    Op::Add => '+',
                    Op::Sub => '-',
                    Op::Mul => '*',
                    Op::Div => '/',
                };
                write!(f, "{lhs} {operator} {rhs}")
            },
        }
    }
}

pub(crate) fn evaluate(expr: Expr) -> Result<Expression, LoxError> {
    let output = match expr {
        Expr::Literal(literal) => match literal {
            Literal::Number(val) => Expression::Number(val),
            Literal::String(val) => Expression::String(val),
            Literal::True => Expression::Bool(true),
            Literal::False => Expression::Bool(false),
            Literal::Nil => Expression::Nil,
        },
        Expr::Unary(operator, expr) => {
            let expr = evaluate(*expr)?;
            match operator {
                Operator::Negate => Expression::Negate(expr.into()),
                Operator::Not => Expression::Not(expr.into()),
                _ => return Err(LoxError::InvalidToken {
                    error: "! or - expected"
                })
            }
        },
        Expr::Binary(lhs, operator, rhs) => {
            match operator {
                Operator::Plus | Operator::Minus | Operator::Mult | Operator::Divide => {
                    let lhs = evaluate(*lhs)?;
                    let rhs = evaluate(*rhs)?;
                    let op = match operator {
                        Operator::Plus => Op::Add,
                        Operator::Minus => Op::Sub,
                        Operator::Mult => Op::Mul,
                        Operator::Divide => Op::Div,
                        _ => unreachable!()
                    };
                    Expression::Op(lhs.into(), op, rhs.into())
                },
                Operator::EqualTo | Operator::NotEqualTo | 
                Operator::LessThan | Operator::LessEqualThan | 
                Operator::GreaterThan | Operator::GreaterEqualThan => {
                    let lhs = evaluate(*lhs)?;
                    let rhs = evaluate(*rhs)?;
                    let equality = match operator {
                        Operator::EqualTo => Equality::Eq,
                        Operator::NotEqualTo => Equality::NotEq,
                        Operator::LessThan => Equality::Lt,
                        Operator::LessEqualThan => Equality::Lte,
                        Operator::GreaterThan => Equality::Gt,
                        Operator::GreaterEqualThan => Equality::Gte,
                        _ => unreachable!()
                    };
                    Expression::Equality(lhs.into(), equality, rhs.into())
                },
                _ => return Err(LoxError::InvalidToken { error: "Invalid binary operation" }),
            }
        },
        Expr::Grouping(expr) => evaluate(*expr)?,
    };

    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::{lexer::lexer::Lexer, parser::parser::parse};

    use super::*;


    fn evaluate_test(source: &str) -> Expression {
        let mut lexer = Lexer::new();
        let _ = lexer.tokenize(source);
        let mut tokens = lexer.get();
        let expression = parse(&mut tokens);
        evaluate(expression).unwrap()
    }

    #[test]
    fn number() {
        let expr = evaluate_test("123");
        assert_eq!(expr.to_string(), "123");
    }

    #[test]
    fn negative_number() {
        let expr = evaluate_test("-123");
        assert_eq!(expr.to_string(), "-123");
    }

    #[test]
    fn bool() {
        let expr = evaluate_test("true");
        assert_eq!(expr.to_string(), "true");

        let expr = evaluate_test("!true");
        assert_eq!(expr.to_string(), "!true");

        let expr = evaluate_test("!false");
        assert_eq!(expr.to_string(), "!false");

        let expr = evaluate_test("!!false");
        assert_eq!(expr.to_string(), "!!false");
    }

    #[test]
    fn addition() {
        let expr = evaluate_test("-2 + -3");
        assert_eq!(expr.to_string(), "-2 + -3");

        let expr = evaluate_test("2 + -3");
        assert_eq!(expr.to_string(), "2 + -3");

        let expr = evaluate_test("2 + -1");
        assert_eq!(expr.to_string(), "2 + -1");

        let expr = evaluate_test("-3 + 2");
        assert_eq!(expr.to_string(), "-3 + 2");

        let expr = evaluate_test("-1 + 2");
        assert_eq!(expr.to_string(), "-1 + 2");

        let expr = evaluate_test("1 + 2 * 3");
        assert_eq!(expr.to_string(), "1 + 2 * 3");
    }

    #[test]
    fn multiplication() {
        let expr = evaluate_test("2 * 2");
        assert_eq!(expr.to_string(), "2 * 2");
    }

    #[test]
    fn subtraction() {
        let expr = evaluate_test("5 - 4");
        assert_eq!(expr.to_string(), "5 - 4");

        let expr = evaluate_test("-5 - 4");
        assert_eq!(expr.to_string(), "-5 - 4");

        let expr = evaluate_test("-5 - -4");
        assert_eq!(expr.to_string(), "-5 - -4");
    }

    #[test]
    fn division() {
        let expr = evaluate_test("5 / 4");
        assert_eq!(expr.to_string(), "5 / 4");
    }

}
