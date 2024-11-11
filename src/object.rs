#[derive(Debug)]
pub(crate) enum Object<'a> {
    Boolean(bool),
    Null,
    Number(f64),
    String(&'a str),
}

impl <'a>ToString for Object<'a> {
    fn to_string(&self) -> String {
        match self {
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".to_string(),
            Object::Number(value) => value.to_string(),
            Object::String(value) => value.to_string(),
        }
    }
}
