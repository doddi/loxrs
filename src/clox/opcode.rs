use super::CloxValue;

#[derive(Debug)]
pub(super) enum Opcode {
    Return,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Constant(CloxValue),
}
