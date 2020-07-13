use crate::eip::{Category, Eip, Status, Type};

use anyhow::Error;
use std::fmt;
use std::fs;

#[derive(Debug, Default)]
pub struct Runner<'a> {
    path: &'a str,
    errors: Vec<(String, Vec<Error>)>,

    // validity count
    valid: u64,
    invalid: u64,

    // statuses count
    draft: u64,
    last_call: u64,
    accepted: u64,
    final_: u64,
    active: u64,
    abandoned: u64,
    superseded: u64,
    rejected: u64,

    // types count
    standards: u64,
    meta: u64,
    informational: u64,

    // categories count
    core: u64,
    erc: u64,
    interface: u64,
    networking: u64,
}

impl<'a> Runner<'a> {
    pub fn new(path: &'a str) -> Self {
        let mut ret = Self::default();
        ret.path = path;
        ret
    }

    pub fn validate(&mut self) {
        match fs::metadata(self.path) {
            Ok(m) => {
                if m.is_file() {
                    self.validate_single(self.path)
                } else {
                    let dir = fs::read_dir(self.path).expect("unable to read dir");
                    for entry in dir {
                        if let Ok(entry) = entry {
                            self.validate_single(entry.path())
                        }
                    }
                }
            }
            Err(e) => panic!(e),
        }
    }

    fn validate_single<P: AsRef<std::path::Path> + Clone>(&mut self, path: P) {
        let res: Result<Eip, Vec<Error>> = fs::read_to_string(path.clone()).unwrap().parse();
        self.count(
            res,
            path.as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    fn count(&mut self, res: Result<Eip, Vec<Error>>, file_name: String) {
        match res {
            Ok(eip) => {
                self.valid += 1;

                match eip.preamble.status {
                    Some(Status::Draft) => self.draft += 1,
                    Some(Status::LastCall) => self.last_call += 1,
                    Some(Status::Accepted) => self.accepted += 1,
                    Some(Status::Final) => self.final_ += 1,
                    Some(Status::Active) => self.active += 1,
                    Some(Status::Abandoned) => self.abandoned += 1,
                    Some(Status::Superseded) => self.superseded += 1,
                    Some(Status::Rejected) => self.rejected += 1,
                    None => (),
                }

                match eip.preamble.ty {
                    Some(Type::Standards) => self.standards += 1,
                    Some(Type::Informational) => self.informational += 1,
                    Some(Type::Meta) => self.meta += 1,
                    None => (),
                }

                match eip.preamble.category {
                    Some(Category::Core) => self.core += 1,
                    Some(Category::Networking) => self.networking += 1,
                    Some(Category::Interface) => self.interface += 1,
                    Some(Category::Erc) => self.erc += 1,
                    None => (),
                }
            }
            Err(e) => {
                self.invalid += 1;
                self.errors.push((file_name, e));
            }
        }
    }
}

impl<'a> fmt::Display for Runner<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in self.errors.iter() {
            let eip = error.0.clone();
            for error in error.1.iter() {
                write!(f, "{}:\t{}\n", eip, error)?;
            }
        }

        write!(f, "\n")?;
        write!(f, "draft: {}, last_call: {}, accepted: {}, final: {}, active: {}, abandonded: {}, superseded: {}, rejected: {}\n", self.draft, self.last_call, self.accepted, self.final_, self.active, self.abandoned, self.superseded, self.rejected)?;
        write!(f, "valid: {}, invalid: {}", self.valid, self.invalid)
    }
}
