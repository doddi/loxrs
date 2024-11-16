use super::{expr::Expr, loxerror::LoxError, token::Token};

#[derive(Debug)]
pub(crate) enum Statement {
    Print(Box<Expr>),
    If(Box<Expr>, Box<Self>, Option<Box<Self>>),
    Expression(Box<Expr>),
    Block(Vec<Statement>),

    Function {
        name: Token,
        args: Vec<Token>,
        body: Vec<Statement>,
    },
    Var { name: Token, initializer: Option<Expr> },
}

impl Statement {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, LoxError> {
        match self {
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::If(condition, if_branch, else_branch) => {
                visitor.visit_if_statement(condition, if_branch, else_branch)
            }
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Block(statement) => visitor.visit_block_statement(statement),
            Statement::Function {name, args, body} => visitor.visit_function_statement(name, args, body),
            Statement::Var { name, initializer } => visitor.visit_var_statement(name, initializer),
        }
    }
}

pub(crate) trait Visitor<R> {
    fn visit_print_statement(&mut self, expr: &Box<Expr>) -> Result<R, LoxError>;
    fn visit_if_statement(
        &mut self,
        condition: &Box<Expr>,
        if_branch: &Box<Statement>,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<R, LoxError>;
    fn visit_expression_statement(&mut self, expr: &Box<Expr>) -> Result<R, LoxError>;
    fn visit_block_statement(&mut self, statements: &Vec<Statement>) -> Result<R, LoxError>;
    fn visit_function_statement(
        &mut self,
        name: &Token,
        args: &Vec<Token>,
        body: &Vec<Statement>,
    ) -> Result<R, LoxError>;
    fn visit_var_statement(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<R, LoxError>;
}
