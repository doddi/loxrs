use super::{
    chunk::Chunk, clox_error::CloxError, compiler::Compiler, opcode::Opcode, stack::Stack,
    CloxValue,
};

pub(super) struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Stack,
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
                        self.stack.push(-value);
                    }
                    Opcode::Add => self.binary_op(|a, b| a + b)?,
                    Opcode::Sub => self.binary_op(|a, b| a - b)?,
                    Opcode::Mul => self.binary_op(|a, b| a * b)?,
                    Opcode::Div => self.binary_op(|a, b| a / b)?,
                },
                None => return Err(CloxError::RuntimeError),
            }

            self.ip += 1;
        }
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), CloxError>
    where
        F: FnOnce(CloxValue, CloxValue) -> CloxValue,
    {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(op(a, b));
        Ok(())
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
