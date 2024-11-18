use clox_error::CloxError;
use vm::Vm;

mod chunk;
mod clox_error;
mod clox_value;
mod compiler;
mod opcode;
mod scanner;
mod stack;
mod string_indexer;
mod token;
mod vm;

pub fn run(content: &str) -> Result<(), CloxError> {
    let mut vm = Vm::new();

    vm.interpret(content)
}
