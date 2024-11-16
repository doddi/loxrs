use std::{cell::RefCell, rc::Rc};

use super::{environment::Environment, interpreter, loxerror::LoxError, object::Object, statement::Statement, token::Token};

#[derive(Debug)]
pub(super) enum Function {
    User {
        name: Token,
        args: Vec<Token>,
        body: Vec<Statement>,
        parent: Rc<RefCell<Environment>>,
    },
}

impl Function {
    pub(crate) fn call(
        &self,
        interpreter: &mut interpreter::Interpreter,
        _args: &Vec<Object>,
    ) -> Result<Object, LoxError> {
        match self {
            Function::User {
                name: _name,
                args: _args,
                body,
                parent: parent,
            } => {
                //let env = Rc::new(RefCell::new());
                //match interpreter.execute_block(body) {
                //    Ok(_) => Ok(Object::Null),
                //    Err(err) => Err(err),
                //}
                todo!()
            },
        }
    }
}
