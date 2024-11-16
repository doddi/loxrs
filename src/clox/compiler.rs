use crate::clox::token::Token;

use super::{clox_error::CloxError, scanner::Scanner};

pub(super) struct Compiler {

}

impl Compiler {
    pub(super) fn new() -> Self {
        Self {
        }
    }

    pub(crate) fn compile(&self, scanner: &Scanner) -> Result<(), CloxError> {
        //loop {
        //    let token = scanner.scan_token()?;
        //    if token.line != self.line {
        //        print!("{:04}", token.line);
        //    }
        //    else {
        //        print!("   | ");
        //    }
        //    println!("{:.2} '{}{}", token.type, token.length, token.start);
        //
        //    if token.token_type == TokeType::Eof {
        //        break;
        //    }
        //}

        todo!()
    }
}
