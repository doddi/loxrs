use std::fmt::Display;

use super::{vm::VmError, CloxValue};

pub(super) struct Stack {
    inner: Vec<CloxValue>,
}

impl Stack {
    pub(super) fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    pub(super) fn push(&mut self, value: CloxValue) {
        self.inner.push(value);
    }

    pub(super) fn pop(&mut self) ->Result<CloxValue, VmError> {
        match self.inner.pop() {
            Some(value) => Ok(value),
            None => Err(VmError::StackUnderflow),
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let _ = write!(f, "           ");
        for value in self.inner.iter() {
            let _ = write!(f, "[{value}]");
        }
        let _ = writeln!(f, "");
        Ok(())
    }
}

