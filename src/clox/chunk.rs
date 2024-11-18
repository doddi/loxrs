use super::{clox_value::CloxValue, opcode::Opcode};

pub(super) struct Chunk {
    opcodes: Vec<Opcode>,
    lines: Vec<usize>,
}

impl Chunk {
    pub(super) fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub(super) fn write_chunk(&mut self, opcode: Opcode, line: usize) {
        self.opcodes.push(opcode);
        self.lines.push(line);
    }

    pub(super) fn get_at(&self, offset: usize) -> Option<&Opcode> {
        self.opcodes.get(offset)
    }

    // Maybe these can go into some kind of visitor pattern or simlpy part of the Display impl
    pub(super) fn disassemble_chunk(&self, source: &str) {
        println!("== {} ==", source);

        self.opcodes.iter().enumerate().for_each(|(idx, opcode)| {
            self.disassemble_instruction(opcode, idx);
        });
    }

    fn disassemble_instruction(&self, opcode: &Opcode, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ");
        } else {
            print!("{:<3} ", self.lines[offset]);
        }

        match opcode {
            Opcode::Return => self.simple_instruction("OP_RETURN", offset),
            Opcode::Constant(value) => self.constant_instruction("OP_CONSTANT", value, offset),
            Opcode::Negate => self.simple_instruction("OP_NEGATE", offset),
            Opcode::Nil => self.simple_instruction("OP_NIL", offset),
            Opcode::True => self.simple_instruction("OP_TRUE", offset),
            Opcode::False => self.simple_instruction("OP_FALSE", offset),
            Opcode::Add => self.simple_instruction("OP_ADD", offset),
            Opcode::Sub => self.simple_instruction("OP_SUB", offset),
            Opcode::Mul => self.simple_instruction("OP_MUL", offset),
            Opcode::Div => self.simple_instruction("OP_DIV", offset),
            Opcode::Not => self.simple_instruction("OP_NOT", offset),
            Opcode::Equal => self.simple_instruction("OP_EQUAL", offset),
            Opcode::Greater => self.simple_instruction("OP_GREATER", offset),
            Opcode::Less => self.simple_instruction("OP_LESS", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(&self, name: &str, value: &CloxValue, offset: usize) -> usize {
        print!("{} '", name);
        self.print_value(value);
        println!("'");
        offset + 1
    }

    pub(crate) fn print_value(&self, value: &CloxValue) {
        print!("{}", value);
    }
}

#[cfg(test)]
mod test {
    use crate::clox::clox_value::CloxValue;

    use super::Chunk;

    #[test]
    fn simple_test() {
        let mut chunk = Chunk::new();

        chunk.write_chunk(super::Opcode::Constant(CloxValue::Number(1.2)), 123);
        chunk.write_chunk(super::Opcode::Negate, 123);
        chunk.write_chunk(super::Opcode::Return, 123);

        chunk.disassemble_chunk("test chunk");
    }
}
