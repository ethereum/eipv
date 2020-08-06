use crate::error::Error;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Context {
    ignore: HashSet<Error>,
    skip: HashSet<String>,
}

impl Context {
    pub fn skip(&mut self, s: &str) {
        self.skip.insert(s.to_string());
    }

    pub fn ignore(&mut self, e: Error) {
        self.ignore.insert(e);
    }

    pub fn should_ignore(&self, e: &Error) -> bool {
        self.ignore.contains(e)
    }

    pub fn should_skip(&self, s: &str) -> bool {
        self.skip.contains(s)
    }
}
