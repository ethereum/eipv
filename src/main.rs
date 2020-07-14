#![allow(unused_variables)]
mod eip;
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
        .get_matches();

    let mut runner = Runner::new(matches.value_of("path").unwrap());
    runner.validate();

    println!("{}", runner);
}
