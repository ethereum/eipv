#![allow(unused_variables)]
mod ctx;
mod eip;
mod error;
mod runner;
mod validators;

use clap::{App, Arg};
use runner::Runner;
use std::process::exit;

fn main() {
    let matches = App::new("eipv")
        .version("0.0.0")
        .about("Validate the structure of Ethereum Improvement Proposals")
        .arg(
            Arg::new("path")
                .takes_value(true)
                .required(true)
                .about("Directory of EIPs or path to a specific EIP"),
        )
        .arg(
            Arg::new("ignore")
                .takes_value(true)
                .short('i')
                .long("ignore")
                .about("Run the validation suite, ignoring the specified errors."),
        )
        .arg(
            Arg::new("skip")
                .takes_value(true)
                .short('s')
                .long("skip")
                .about("Skip validation of the specified files."),
        )
        .get_matches();

    let runner = Runner::new(
        matches.value_of("path").unwrap(),
        matches.value_of("ignore"),
        matches.value_of("skip"),
    );

    match runner {
        Ok(mut r) => {
            r.validate();
            println!("{}", r);

            if r.invalid() != 0 {
                exit(1)
            }
        }
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}
