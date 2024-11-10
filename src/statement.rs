use crate::{expr::Expr, loxerror::LoxError, object::Object};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Print(Box<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Self>, Option<Box<Self>>),
    Expression(Box<Expr<'a>>),
}


impl Statement<'_> {
    pub(crate) fn accept<'v: 's, 's>(&'s self, visitor: &'v mut dyn Visitor) -> Result<Object, LoxError> {
        match self {
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::If(condition, if_branch, else_branch) => visitor.visit_if_statement(condition, if_branch, else_branch),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
        }
    }

}

pub(crate) trait Visitor<'a> {
    fn visit_print_statement(&mut self, expr: &Box<Expr>) -> Result<Object, LoxError>;
    fn visit_if_statement(&mut self, condition: &Box<Expr>, if_branch: &Box<Statement>, else_branch: &Option<Box<Statement>>) -> Result<Object, LoxError>;
    fn visit_expression_statement(&mut self, expr: &Box<Expr>) -> Result<Object, LoxError>;
}

