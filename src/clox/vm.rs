use tracing::trace;

use super::{
    chunk::Chunk, clox_error::CloxError, clox_value::CloxValue, compiler::Compiler, opcode::Opcode,
    stack::Stack,
};

pub(super) struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Stack,
}

macro_rules! binary_op{
    ( $vm:ident, $value_constructor:expr, $op:tt ) => {
        {
            if !$vm.stack.peek(0).is_number() || !$vm.stack.peek(1).is_number() {
                return Err(CloxError::RuntimeError);
            }

            let b = $vm.stack.pop().expect("").as_number();
            let a = $vm.stack.pop().expect("").as_number();
            $vm.stack.push($value_constructor(a $op b));
        }
    };
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Stack::new(),
        }
    }

    pub(super) fn interpret(&mut self, content: &str) -> Result<(), CloxError> {
        let mut compiler = Compiler::new();
        match compiler.compile(content) {
            Ok(chunk) => {
                self.chunk = chunk;
                self.run()
            }
            Err(_) => Err(CloxError::CompileError),
        }
    }

    fn run(&mut self) -> Result<(), CloxError> {
        loop {
            match self.chunk.get_at(self.ip) {
                Some(opcode) => match opcode {
                    Opcode::Return => {
                        self.chunk.print_value(&self.stack.pop()?);
                        println!("");
                        return Ok(());
                    }
                    Opcode::Constant(constant) => self.stack.push(*constant),
                    Opcode::Negate => {
                        let value = self.stack.pop()?;
                        match value {
                            CloxValue::Number(number) => {
                                self.stack.push(CloxValue::Number(-number))
                            }
                            _ => unreachable!(),
                        }
                    }
                    Opcode::Nil => self.stack.push(CloxValue::Nil),
                    Opcode::True => self.stack.push(CloxValue::Boolean(true)),
                    Opcode::False => self.stack.push(CloxValue::Boolean(false)),
                    Opcode::Add => binary_op!(self, CloxValue::new_num, +),
                    Opcode::Sub => binary_op!(self, CloxValue::new_num, -),
                    Opcode::Mul => binary_op!(self, CloxValue::new_num, *),
                    Opcode::Div => binary_op!(self, CloxValue::new_num, /),
                    Opcode::Not => {
                        let value = self.stack.pop()?;
                        trace!("No operation on: {:?}", value);
                        match value {
                            CloxValue::Boolean(b) => {
                                self.stack.push(CloxValue::Boolean(self.is_falsey(b)))
                            }
                            CloxValue::Number(_) => unreachable!("Not allowed to not on a number"),
                            CloxValue::Nil => self.stack.push(CloxValue::Boolean(false)),
                        }
                    }
                    Opcode::Equal => {
                        let a = self.stack.pop()?;
                        let b = self.stack.pop()?;
                        self.stack.push(CloxValue::new_bool(a == b));
                    }
                    Opcode::Greater => binary_op!(self, CloxValue::new_bool, >),
                    Opcode::Less => binary_op!(self,CloxValue::new_bool, <),
                },
                None => return Err(CloxError::RuntimeError),
            }

            self.ip += 1;
        }
    }

    fn is_falsey(&self, value: bool) -> bool {
        !value
    }
}

#[cfg(test)]
mod test {
    use super::{Chunk, Opcode, Vm};

    #[test]
    fn test_vm() {
        let mut chunk = Chunk::new();
        chunk.write_chunk(Opcode::Constant(1.2), 123);
        chunk.write_chunk(Opcode::Constant(3.4), 123);

        chunk.write_chunk(Opcode::Add, 123);

        chunk.write_chunk(Opcode::Constant(5.6), 123);
        chunk.write_chunk(Opcode::Div, 123);

        chunk.write_chunk(Opcode::Negate, 123);
        chunk.write_chunk(Opcode::Return, 123);

        let mut vm = Vm::new();
        let _ = vm.run();
    }
}
