use clox_error::CloxError;
use vm::Vm;

mod chunk;
mod opcode;
mod vm;
mod stack;
mod compiler;
mod scanner;
mod token;
mod string_indexer;
mod clox_error;

type CloxValue = f64;

pub fn run(content: &str) -> Result<(), CloxError> {
    let mut vm = Vm::new();

    vm.interpret(content)
}
