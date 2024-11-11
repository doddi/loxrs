use crate::{expr::{self, Expr, Literal}, loxerror::LoxError, object::Object, statement::{self, Statement}};


pub(crate) struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, statements: &Vec<Statement<'_>>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Statement<'_>) -> Result<(), LoxError> {
        statement.accept(self)
    }

    fn evaluate<'a>(&mut self, expr: &Expr<'a>) -> Result<Object<'a>, LoxError> {
       expr.accept(self) 
    }
}

impl <'v>expr::Visitor<'v> for Interpreter {
    //fn visit_binary_expression(&mut self, lhs: &Expr, operator: &expr::Operator, rhs: &Expr) -> Result<Object, LoxError> {
    //    let _lhs = self.evaluate(lhs)?; 
    //    let _rhs = self.evaluate(rhs)?; 
    //
    //    match operator {
    //        expr::Operator::EqualTo => todo!(),
    //        expr::Operator::NotEqualTo => todo!(),
    //        expr::Operator::LessThan => todo!(),
    //        expr::Operator::LessEqualThan => todo!(),
    //        expr::Operator::GreaterThan => todo!(),
    //        expr::Operator::GreaterEqualThan => todo!(),
    //        expr::Operator::Plus => todo!(),
    //        expr::Operator::Minus => todo!(),
    //        expr::Operator::Mult => todo!(),
    //        expr::Operator::Divide => todo!(),
    //        expr::Operator::Negate => todo!(),
    //        expr::Operator::Not => todo!(),
    //    }
    //}

    fn visit_literal_expression<'output>(&self, literal: &Literal<'output>) -> Result<Object<'output>, LoxError> {
        match literal {
            Literal::Number(val) => Ok(Object::Number(*val)),
            Literal::String(val) => Ok(Object::String(val)),
            Literal::Bool(val) => Ok(Object::Boolean(*val)),
            Literal::Nil => Ok(Object::Null),
        }
    }

    fn visit_unary_expression(&mut self, operator: &expr::Operator, expr: &Expr) -> Result<Object<'v>, LoxError> {
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

    //fn visit_grouping_expression(&mut self, _expr: &Expr) -> Result<Object, LoxError> {
    //    todo!()
    //}
}

impl statement::Visitor for Interpreter {
    fn visit_print_statement<'output>(&mut self, expr: &Box<Expr<'output>>) -> Result<(), LoxError> {
        let result = self.evaluate(expr)?;
        println!("{}", result.to_string());
        Ok(())
    }

    fn visit_if_statement<'con, 'output>(&mut self, _condition: &Box<Expr<'con>>, _if_branch: &Box<Statement<'output>>, _else_branch: &Option<Box<Statement<'output>>>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_expression_statement<'output>(&mut self, expr: &Box<Expr<'output>>) -> Result<(), LoxError> {
        self.evaluate(expr)?;
        Ok(())
    }
}
