use crate::{interpreter, loxerror::LoxError, object::Object, statement::Statement, token::Token};

#[derive(Debug)]
pub(crate) enum Function<'a> {
    User {
        name: Token<'a>,
        args: Vec<Token<'a>>,
        body: Vec<Statement<'a>>,
    }
}


impl <'a>Function<'a> {
    pub(crate) fn call(&self, interpreter: &mut interpreter::Interpreter, _args: &Vec<Object>) -> Result<Object, LoxError> {
        match self {
            Function::User { 
                name: _name, 
                args: _args, 
                body } => {

                match interpreter.execute_block(body) {
                    Ok(_) => Ok(Object::Null),
                    Err(err) => Err(err),
                }
            },
        }
    }
}
