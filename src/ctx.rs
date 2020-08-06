use crate::error::Error;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Context {
    exclude: HashSet<Error>,
}

impl Context {
    pub fn exclude(&mut self, e: Error) {
        self.exclude.insert(e);
    }

    pub fn should_exclude(&self, e: &Error) -> bool {
        self.exclude.contains(e)
    }

    pub fn difference(&self, s: HashSet<Error>) -> HashSet<Error> {
        s.difference(&self.exclude).cloned().collect()
    }
}
