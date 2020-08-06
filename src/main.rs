#![allow(unused_variables)]
mod ctx;
mod eip;
mod error;
mod runner;
mod validators;

use clap::{App, Arg};
use runner::Runner;

fn main() {
    let matches = App::new("eipv")
        .version("0.0.0")
        .about("Validate the structure of Ethereum Improvement Proposals")
        .arg(
            Arg::with_name("path")
                .takes_value(true)
                .required(true)
                .about("Directory of EIPs or path to a specific EIP"),
        )
        .arg(
            Arg::with_name("ignore")
                .takes_value(true)
                .short('i')
                .long("ignore")
                .about("Run the validation suite, ignoring the specified errors."),
        )
        .get_matches();

    let runner = Runner::new(
        matches.value_of("path").unwrap(),
        matches.value_of("ignore"),
    );

    match runner {
        Ok(mut r) => {
            r.validate();
            println!("{}", r);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
