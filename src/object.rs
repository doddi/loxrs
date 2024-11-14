use crate::function::Function;


#[derive(Debug)]
pub(crate) enum Object<'src> {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),

    Callable(Function<'src>),
}

impl <'src>ToString for Object<'src> {
    fn to_string(&self) -> String {
        match self {
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".to_string(),
            Object::Number(value) => value.to_string(),
            Object::String(value) => value.to_string(),
            Object::Callable(_function) => todo!(),
        }
    }
}
