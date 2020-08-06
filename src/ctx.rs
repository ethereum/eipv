use crate::error::Error;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Context {
    ignore: HashSet<Error>,
}

impl Context {
    pub fn ignore(&mut self, e: Error) {
        self.ignore.insert(e);
    }

    pub fn should_ignore(&self, e: &Error) -> bool {
        self.ignore.contains(e)
    }
}
