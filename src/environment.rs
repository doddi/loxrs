use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

// An environment gives you access to scoped variables
#[derive(Debug)]
pub(crate) struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }
}
