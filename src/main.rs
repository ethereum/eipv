mod eip;
mod validators;

use eip::{Category, Eip, Status, Type};

use anyhow::Error;
use std::env::args;
use std::fmt;
use std::fs;
use std::io;
use std::process::exit;

fn main() {
    let mut args = args();

    if args.len() != 2 {
        println!("Usage: eipv [dir]");
        exit(1)
    }

    let dir = args.nth(1).unwrap();

    let mut runner = Runner::new(dir);
    runner.validate();

    println!("{}", runner);
}

#[derive(Debug, Default)]
struct Runner {
    dir: String,
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

impl Runner {
    pub fn new(dir: String) -> Self {
        let mut ret = Self::default();
        ret.dir = dir;
        ret
    }

    pub fn validate(&mut self) {
        let dir = match fs::read_dir(self.dir.clone()) {
            Ok(d) => d,
            Err(e) => panic!(e),
        };

        for entry in dir {
            if let Ok(entry) = entry {
                let res: Result<Eip, Vec<Error>> =
                    fs::read_to_string(entry.path()).unwrap().parse();

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
                        self.errors
                            .push((entry.file_name().into_string().unwrap(), e));
                    }
                }
            }
        }
    }
}

impl fmt::Display for Runner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in self.errors.iter() {
            let eip = error.0.clone();
            for error in error.1.iter() {
                write!(f, "{}: {}\n", eip, error)?;
            }
        }

        write!(f, "\n")?;
        write!(f, "draft: {}, last_call: {}, accepted: {}, final: {}, active: {}, abandonded: {}, superseded: {}, rejected: {}\n", self.draft, self.last_call, self.accepted, self.final_, self.active, self.abandoned, self.superseded, self.rejected)?;
        write!(f, "valid: {}, invalid: {}", self.valid, self.invalid)
    }
}
