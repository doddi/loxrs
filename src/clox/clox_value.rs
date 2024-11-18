use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub(super) enum CloxValue {
    Boolean(bool),
    Number(f64),
    Nil,
}

impl Display for CloxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloxValue::Boolean(value) => write!(f, "{value}"),
            CloxValue::Number(value) => write!(f, "{value}"),
            CloxValue::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for CloxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl CloxValue {
    pub(super) fn new_bool(value: bool) -> Self {
        CloxValue::Boolean(value)
    }

    pub(super) fn new_num(value: f64) -> Self {
        CloxValue::Number(value)
    }

    pub(super) fn is_number(&self) -> bool {
        matches!(self, CloxValue::Number(_))
    }

    pub(super) fn as_number(&self) -> f64 {
        match self {
            CloxValue::Number(val) => *val,
            _ => unreachable!("not a number type"),
        }
    }
}
