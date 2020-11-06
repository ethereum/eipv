use crate::ctx::Context;
use crate::eip::{Category, Eip, Status, Type};
use crate::error::Error;

use anyhow::Result;
use std::fmt;
use std::fs;

#[derive(Debug, Default)]
pub struct Runner<'a> {
    path: &'a str,
    ctx: Context,
    errors: Vec<(String, Vec<Error>)>,

    // validity count
    valid: u64,
    invalid: u64,

    // statuses count
    draft: u64,
    review: u64,
    last_call: u64,
    final_: u64,
    stagnant: u64,
    withdrawn: u64,
    living: u64,

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
    pub fn new(path: &'a str, ignore: Option<&'a str>, skip: Option<&'a str>) -> Result<Self> {
        let mut ret = Self::default();
        ret.path = path;

        if let Some(ignore) = ignore {
            for i in ignore.split(',') {
                Error::from_str(i).and_then(|v| Ok(ret.ctx.ignore(v)))?;
            }
        }

        if let Some(skip) = skip {
            for s in skip.split(',') {
                ret.ctx.skip(s);
            }
        }

        Ok(ret)
    }

    pub fn invalid(&self) -> u64 {
        self.invalid
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
        if !self
            .ctx
            .should_skip(path.as_ref().file_name().unwrap().to_str().unwrap())
        {
            let res: Result<Eip, Vec<Error>> = Eip::from_str(
                &self.ctx,
                &fs::read_to_string(path.clone())
                    .unwrap()
                    // normalize newlines
                    .replace("\r\n", "\n"),
            );
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
    }

    fn count(&mut self, res: Result<Eip, Vec<Error>>, file_name: String) {
        match res {
            Ok(eip) => {
                self.valid += 1;

                match eip.preamble.status {
                    Some(Ok(Status::Draft)) => self.draft += 1,
                    Some(Ok(Status::Review)) => self.review += 1,
                    Some(Ok(Status::LastCall)) => self.last_call += 1,
                    Some(Ok(Status::Final)) => self.final_ += 1,
                    Some(Ok(Status::Stagnant)) => self.stagnant += 1,
                    Some(Ok(Status::Withdrawn)) => self.withdrawn += 1,
                    Some(Ok(Status::Living)) => self.living += 1,
                    _ => (),
                }

                match eip.preamble.ty {
                    Some(Ok(Type::Standards)) => self.standards += 1,
                    Some(Ok(Type::Informational)) => self.informational += 1,
                    Some(Ok(Type::Meta)) => self.meta += 1,
                    _ => (),
                }

                match eip.preamble.category {
                    Some(Ok(Category::Core)) => self.core += 1,
                    Some(Ok(Category::Networking)) => self.networking += 1,
                    Some(Ok(Category::Interface)) => self.interface += 1,
                    Some(Ok(Category::Erc)) => self.erc += 1,
                    _ => (),
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
                write!(f, "{}:\t{}\n", eip, error.human_readable())?;
            }
        }

        write!(f, "\n")?;
        write!(f, "draft: {}, review: {}, last_call: {}, final: {}, stagnant: {}, withdrawn: {}, living: {}\n", self.draft, self.review, self.last_call, self.final_, self.stagnant, self.withdrawn, self.living)?;
        write!(f, "valid: {}, invalid: {}", self.valid, self.invalid)
    }
}
