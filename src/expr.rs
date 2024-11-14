use std::fmt::Display;

use crate::{loxerror::LoxError, object::Object};

#[derive(Debug)]
pub(crate) enum Expr<'src> {
    Literal(Literal<'src>),
    Unary(Operator, Box<Expr<'src>>),
    Binary(Box<Expr<'src>>, Operator, Box<Expr<'src>>),
    Grouping(Box<Expr<'src>>),

    Call { callee: Box<Expr<'src>>, args: Vec<Expr<'src>>},
}

impl <'src>Display for Expr<'src> {
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

impl <'src>Expr<'src> {
    pub fn accept<'exp, 'visit>(&'exp self, visitor: &'visit mut dyn Visitor<Output = Object<'exp>>) -> Result<Object<'exp>, LoxError> {
        match self {
            Expr::Literal(literal) => visitor.visit_literal_expression(literal),
            Expr::Unary(op, expr) => visitor.visit_unary_expression(op, expr),
            Expr::Binary(lhs, op, rhs) => visitor.visit_binary_expression(lhs, op, rhs),
            Expr::Grouping(expr) => visitor.visit_grouping_expression(expr),
            Expr::Call { callee, args } => visitor.visit_function_expression(callee, &args),
        }
    }
}

pub(crate) trait Visitor {
    type Output;

    fn visit_binary_expression<'s, 'exp,'src>(&'s mut self, lhs: &'exp Expr<'src>, operator: &'exp Operator, rhs: &'exp Expr<'src>) -> Result<Self::Output, LoxError>;
    fn visit_literal_expression<'s, 'exp, 'src>(&'s self, literal: &'exp Literal<'src>) -> Result<Self::Output, LoxError>;
    fn visit_unary_expression<'s, 'exp, 'src>(&'s mut self, operator: &'exp Operator, expr: &'exp Expr<'src>) -> Result<Self::Output, LoxError>;
    fn visit_grouping_expression<'s, 'exp, 'src>(&'s mut self, expr: &'exp Expr<'src>) -> Result<Self::Output, LoxError>;

    fn visit_function_expression<'s, 'exp, 'arg, 'src>(&'s mut self, callee: &'exp Expr<'src>, args: &'exp Vec<Expr<'src>>) -> Result<Self::Output, LoxError>; 
}

#[derive(Debug)]
pub(crate) enum Literal<'src> {
    Number(f64),
    String(&'src str),
    Bool(bool),
    Nil,
}

impl <'src>Display for Literal<'src> {
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
