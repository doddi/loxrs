use std::fmt::Display;

use crate::loxerror::LoxError;

#[derive(Debug)]
pub(crate) enum Expr<'a> {
    Literal(Literal<'a>),
    Unary(Operator, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Operator, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),

    Call { callee: Box<Expr<'a>>, args: Vec<Expr<'a>>},
}

impl <'a>Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Unary(op, rhs) => write!(f, "{}{}", op, rhs),
            Expr::Binary(lhs, op, rhs) => write!(f, "{} {} {}", op, lhs, rhs),
            Expr::Grouping(expression) => write!(f, "({})", expression),
            Expr::Call { callee, args } => write!(f, "{}({:?})", callee, args),
        };
        Ok(())
    }
}

impl <'a>Expr<'a> {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, LoxError> {
        match self {
            Expr::Literal(literal) => visitor.visit_literal_expression(literal),
            Expr::Unary(op, expr) => visitor.visit_unary_expression(op, expr),
            Expr::Binary(lhs, op, rhs) => visitor.visit_binary_expression(lhs, op, rhs),
            Expr::Grouping(expr) => visitor.visit_grouping_expression(expr),
            Expr::Call { callee, args } => visitor.visit_function_expression(callee, &args),
        }
    }
}

pub(crate) trait Visitor<R> {
    fn visit_binary_expression(&mut self, lhs: &Expr, operator: &Operator, rhs: &Expr) -> Result<R, LoxError>;
    fn visit_literal_expression(&self, literal: &Literal) -> Result<R, LoxError>;
    fn visit_unary_expression(&mut self, operator: &Operator, expr: &Expr) -> Result<R, LoxError>;
    fn visit_grouping_expression(&mut self, expr: &Expr) -> Result<R, LoxError>;

    fn visit_function_expression(&mut self, callee: &Expr, args: &Vec<Expr>) -> Result<R, LoxError>; 
}

#[derive(Debug)]
pub(crate) enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Bool(bool),
    Nil,
}

impl <'a>Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Literal::Number(val) => write!(f, "{}", val),
            Literal::String(val) => write!(f, "{}", val),
            Literal::Bool(val) => write!(f, "{val}"),
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
