use crate::{expr::Expr, loxerror::LoxError, token::Token};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Print(Box<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Self>, Option<Box<Self>>),
    Expression(Box<Expr<'a>>),
    Block(Vec<Statement<'a>>),

    Function { name: Token<'a>, args: Vec<&'a Token<'a>>, body: Vec<Statement<'a>>},
}


impl <'a>Statement<'a> {
    pub(crate) fn accept<'visit, 'stmnt, R>(&'stmnt self, visitor: &'visit mut dyn Visitor<R>) -> Result<R, LoxError> {
        match self {
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::If(condition, if_branch, else_branch) => visitor.visit_if_statement(condition, if_branch, else_branch),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Block(statement) => visitor.visit_block_statement(statement),
            Statement::Function { name: _name, args: _args, body: _body } => todo!(),
        }
    }

}

pub(crate) trait Visitor<R> {
    fn visit_print_statement<'s, 'exp, 'src>(
        &'s mut self, 
        expr: &'exp Box<Expr<'src>>
    ) -> Result<R, LoxError>;

    fn visit_if_statement<'ifl, 'ell, 'con, 'src>(
        &mut self,
        condition: &'con Box<Expr<'src>>,
        if_branch: &'ifl Box<Statement<'src>>, 
        else_branch: &'ell Option<Box<Statement<'src>>>
    ) -> Result<R, LoxError>;

    fn visit_expression_statement<'s, 'exp, 'src>(
        &'s mut self, 
        expr: &'exp Box<Expr<'src>>
    ) -> Result<R, LoxError>;

    fn visit_block_statement<'s, 'stmnt, 'src>(
        &'s mut self, 
        statements: &'stmnt Vec<Statement<'src>>
    ) -> Result<R, LoxError>;

    fn visit_function_statement<'s, 'args, 'body, 'src>(
        &'s mut self,
        name: Token<'src>, 
        args: &'args Vec<Statement<'src>>, 
        body: &'body Vec<Statement<'src>>
    ) -> Result<R, LoxError>;
}

