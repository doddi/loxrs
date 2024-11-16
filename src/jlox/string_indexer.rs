use tracing::trace;

use super::loxerror::LoxError;

#[derive(Debug, PartialEq)]
pub(crate) struct StringLocation {
    pub(super) start: usize,
    pub(super) end: usize,
}

pub(super) type StringId = usize;

#[derive(Debug)]
pub(super) struct StringIndexer<'src> {
    refs: Vec<StringLocation>,
    content: &'src str,
}

impl<'src> StringIndexer<'src> {
    pub(super) fn new(content: &'src str) -> Self {
        Self {
            refs: Vec::new(),
            content,
        }
    }

    pub(super) fn add_string(&mut self, start: usize, end: usize) -> StringId {
        trace!("Adding string with start: {} end: {}", start, end);
        self.refs.push(StringLocation { start, end });
        self.refs.len() - 1
    }

    pub(super) fn get_string(&self, string_id: StringId) -> Result<&'src str, LoxError> {
        if string_id >= self.refs.len() {
            return Err(LoxError::StringIndexOutOfBouds);
        }
        let string_ref = &self.refs[string_id];

        Ok(&self.content[string_ref.start..=string_ref.end])
    }
}
