use super::clox_value::CloxValue;

#[derive(Debug)]
pub(super) enum Opcode {
    Return,
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Sub,
    Mul,
    Div,
    Not,
    Negate,
    Constant(CloxValue),
}
