use super::{
    expr::{self, Expr, Literal},
    loxerror::LoxError,
    object::Object,
    statement::{self, Statement}, token::Token,
};

pub(super) struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, statements: &Vec<Statement>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), LoxError> {
        statement.accept(self)
    }

    fn evaluate<'a>(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    // ------------------------------------------------
    // Binary Operations
    // ------------------------------------------------
    fn add(&self, lhs: Object, rhs: Object) -> Result<Object, LoxError> {
        match (lhs, rhs) {
            (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
            (Object::String(l), Object::String(r)) => Ok(Object::String(format!("{l}{r}"))),
            _ => Err(LoxError::InterpreterStatement),
        }
    }

    fn truthy(&self, condition: &Object) -> bool {
        match condition {
            Object::Boolean(val) => *val,
            Object::Null => false,
            _ => true,
        }
    }

    pub(crate) fn execute_block(&mut self, statements: &Vec<Statement>) -> Result<(), LoxError> {
        Ok(for statement in statements {
            self.execute(statement)?
        })
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_binary_expression(
        &mut self,
        lhs: &Expr,
        operator: &expr::Operator,
        rhs: &Expr,
    ) -> Result<Object, LoxError> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        match operator {
            expr::Operator::EqualTo => todo!(),
            expr::Operator::NotEqualTo => todo!(),
            expr::Operator::LessThan => todo!(),
            expr::Operator::LessEqualThan => todo!(),
            expr::Operator::GreaterThan => todo!(),
            expr::Operator::GreaterEqualThan => todo!(),
            expr::Operator::Plus => self.add(lhs, rhs),
            expr::Operator::Minus => todo!(),
            expr::Operator::Mult => todo!(),
            expr::Operator::Divide => todo!(),
            expr::Operator::Negate => todo!(),
            expr::Operator::Not => todo!(),
        }
    }

    fn visit_literal_expression(&self, literal: &Literal) -> Result<Object, LoxError> {
        match literal {
            Literal::Number(val) => Ok(Object::Number(*val)),
            Literal::String(val) => Ok(Object::String(val.to_string())),
            Literal::Bool(val) => Ok(Object::Boolean(*val)),
            Literal::Nil => Ok(Object::Null),
        }
    }

    fn visit_unary_expression(
        &mut self,
        operator: &expr::Operator,
        expr: &Expr,
    ) -> Result<Object, LoxError> {
        let result = self.evaluate(expr)?;
        match operator {
            expr::Operator::Negate => match result {
                Object::Number(num) => Ok(Object::Number(-num.clone())),
                _ => Err(LoxError::InterpreterExpression),
            },
            expr::Operator::Not => match result {
                Object::Boolean(value) => Ok(Object::Boolean(!value)),
                _ => Err(LoxError::InterpreterExpression),
            },
            _ => Err(LoxError::InterpreterExpression),
        }
    }

    fn visit_grouping_expression(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        self.evaluate(expr)
    }

    fn visit_function_expression(
        &mut self,
        callee: &Expr,
        args: &Vec<Expr>,
    ) -> Result<Object, LoxError> {
        let callee = self.evaluate(callee)?;

        let arg_values: Result<Vec<Object>, LoxError> =
            args.into_iter().map(|arg| self.evaluate(arg)).collect();
        let evaluated_args = arg_values?;

        match callee {
            Object::Callable(function) => function.call(self, &evaluated_args),
            _ => return Err(LoxError::InterpreterExpression),
        }
    }
}

impl statement::Visitor<()> for Interpreter {
    fn visit_print_statement<'output>(&mut self, expr: &Box<Expr>) -> Result<(), LoxError> {
        let result = self.evaluate(expr)?;
        println!("{}", result.to_string());
        Ok(())
    }

    fn visit_if_statement<'con, 'output>(
        &mut self,
        condition: &Box<Expr>,
        if_branch: &Box<Statement>,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), LoxError> {
        let condition = self.evaluate(condition)?;
        if self.truthy(&condition) {
            return self.execute(if_branch);
        } else if let Some(other) = else_branch {
            self.execute(other)?;
        }

        Ok(())
    }

    fn visit_expression_statement<'output>(&mut self, expr: &Box<Expr>) -> Result<(), LoxError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_block_statement(&mut self, statements: &Vec<Statement>) -> Result<(), LoxError> {
        self.execute_block(statements)?;
        Ok(())
    }

    fn visit_function_statement(
        &mut self,
        name: &Token,
        args: &Vec<Token>,
        body: &Vec<Statement>,
    ) -> Result<(), LoxError> {
        Ok(())
    }

    fn visit_var_statement(
        &mut self, 
        name: &Token, 
        initializer: &Option<Expr>
    ) -> Result<(), LoxError> {
        Ok(())
    }
}
