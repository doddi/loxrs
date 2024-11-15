use super::{opcode::{self, Opcode}, CloxValue};

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
    fn disassemble_chunk(&self, source: &str) {
        println!("== {} ==", source);

        self.opcodes.iter().enumerate().for_each(|(idx, opcode)| {
            self.disassemble_instruction(opcode, idx);
        });
    }

    fn disassemble_instruction(&self, opcode: &Opcode, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ");
        }
        else {
            print!("{:<3} ", self.lines[offset]);
        }

        match opcode {
            Opcode::Return => self.simple_instruction("OP_RETURN", offset),
            Opcode::Constant(value) => self.constant_instruction("OP_CONSTANT", &value, offset),
            Opcode::Negate => self.simple_instruction("OP_NEGATE", offset),
            Opcode::Add => self.simple_instruction("OP_ADD", offset),
            Opcode::Sub => self.simple_instruction("OP_SUB", offset),
            Opcode::Mul => self.simple_instruction("OP_MUL", offset),
            Opcode::Div => self.simple_instruction("OP_DIV", offset),
        } 
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        return offset + 1
    }

    fn constant_instruction(&self, name: &str, value: &CloxValue, offset: usize) -> usize {
        print!("{} '", name);
        self.print_value(value);
        println!("'");
        return offset + 1 
    }

    pub(crate) fn print_value(&self, value: &CloxValue) {
        print!("{}", value);
    }
}

#[cfg(test)]
mod test {
    use super::Chunk;


    #[test]
    fn simple_test() {
        let mut chunk = Chunk::new();

        chunk.write_chunk(super::Opcode::Constant(1.2), 123);
        chunk.write_chunk(super::Opcode::Negate, 123);
        chunk.write_chunk(super::Opcode::Return, 123);

        chunk.disassemble_chunk("test chunk");
    }
}
