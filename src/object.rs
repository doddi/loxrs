#[derive(Debug)]
pub(crate) enum Object<'a> {
    Boolean(bool),
    Null,
    Number(f64),
    String(&'a str),
}
