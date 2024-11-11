use std::fmt::Display;

use crate::{loxerror::LoxError, object::Object};

#[derive(Debug)]
pub(crate) enum Expr<'a> {
    Literal(Literal<'a>),
    Unary(Operator, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Operator, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
}

impl <'a>Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Unary(op, rhs) => write!(f, "{}{}", op, rhs),
            Expr::Binary(lhs, op, rhs) => write!(f, "{} {} {}", op, lhs, rhs),
            Expr::Grouping(expression) => write!(f, "({})", expression),
        };
        Ok(())
    }
}

impl <'a>Expr<'a> {
    pub fn accept<'s, 'v: 'a>(&'s self, visitor: &mut dyn Visitor<'v>) -> Result<Object<'a>, LoxError> {
        match self {
            Expr::Literal(literal) => visitor.visit_literal_expression(literal),
            Expr::Unary(op, expr) => visitor.visit_unary_expression(op, expr),
            Expr::Binary(_lhs, _op, _rhs) => todo!(),
            Expr::Grouping(_expr) => todo!(),
        }
    }
}

pub(crate) trait Visitor<'v> {
    //fn visit_binary_expression<'output>(&mut self, lhs: &Expr<'output>, operator: &Operator, rhs: &Expr<'output>) -> Result<Object<'output>, LoxError>;
    fn visit_literal_expression<'output>(&self, literal: &Literal<'output>) -> Result<Object<'output>, LoxError>;
    fn visit_unary_expression(&mut self, operator: &Operator, expr: &Expr) -> Result<Object<'v>, LoxError>;
    //fn visit_grouping_expression(&mut self, expr: &Expr<'output>) -> Result<Object<'output>, LoxError>;
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
