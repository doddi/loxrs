use crate::{expr::{self, Expr, Literal}, loxerror::LoxError, object::Object, statement::Statement};


pub(crate) struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, statements: &Vec<Statement<'_>>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&self, statement: &Statement<'_>) -> Result<(), LoxError> {
        statement.accept(self);
        todo!()
    }

    fn evaluate(&self, expr: &Expr<'_>) -> Result<Object<'_>, LoxError> {
       expr.accept(self) 
    }
}

impl <'l>expr::Visitor<'l> for Interpreter {
    fn visit_binary_expression(&mut self, lhs: &Expr, operator: &expr::Operator, rhs: &Expr) -> Result<Object, LoxError> {
        let _lhs = self.evaluate(lhs)?; 
        let _rhs = self.evaluate(rhs)?; 

        match operator {
            expr::Operator::EqualTo => todo!(),
            expr::Operator::NotEqualTo => todo!(),
            expr::Operator::LessThan => todo!(),
            expr::Operator::LessEqualThan => todo!(),
            expr::Operator::GreaterThan => todo!(),
            expr::Operator::GreaterEqualThan => todo!(),
            expr::Operator::Plus => todo!(),
            expr::Operator::Minus => todo!(),
            expr::Operator::Mult => todo!(),
            expr::Operator::Divide => todo!(),
            expr::Operator::Negate => todo!(),
            expr::Operator::Not => todo!(),
        }
    }

    fn visit_literal_expression(&mut self, literal: &Literal<'l>) -> Result<Object<'l>, LoxError> {
        match literal {
            Literal::Number(val) => Ok(Object::Number(*val)),
            Literal::String(val) => Ok(Object::String(val)),
            Literal::Bool(val) => Ok(Object::Boolean(*val)),
            Literal::Nil => Ok(Object::Null),
        }
    }

    fn visit_unary_expression(&mut self, _operator: &expr::Operator, _expr: &Expr) -> Result<Object, LoxError> {
        todo!()
    }

    fn visit_grouping_expression(&mut self, _expr: &Expr) -> Result<Object, LoxError> {
        todo!()
    }
}

