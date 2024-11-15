use super::CloxValue;

pub(super) enum Opcode {
    Return,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Constant(CloxValue),
}
