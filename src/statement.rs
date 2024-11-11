use crate::{expr::Expr, loxerror::LoxError};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Print(Box<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Self>, Option<Box<Self>>),
    Expression(Box<Expr<'a>>),
    Block(Vec<Statement<'a>>),
}


impl Statement<'_> {
    pub(crate) fn accept<'v: 's, 's, R>(&'s self, visitor: &'v mut dyn Visitor<R>) -> Result<R, LoxError> {
        match self {
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::If(condition, if_branch, else_branch) => visitor.visit_if_statement(condition, if_branch, else_branch),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Block(statement) => visitor.visit_block_statement(statement),
        }
    }

}

pub(crate) trait Visitor<R> {
    fn visit_print_statement<'output>(&mut self, expr: &Box<Expr<'output>>) -> Result<R, LoxError>;
    fn visit_if_statement<'con, 'output>(&mut self,
        condition: &Box<Expr<'con>>,
        if_branch: &Box<Statement<'output>>, 
        else_branch: &Option<Box<Statement<'output>>>) -> Result<R, LoxError>;
    fn visit_expression_statement<'output>(&mut self, expr: &Box<Expr<'output>>) -> Result<R, LoxError>;
    fn visit_block_statement(&mut self, statements: &Vec<Statement>) -> Result<R, LoxError>;
}

