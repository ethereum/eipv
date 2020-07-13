#![feature(str_strip)]

mod eip;
mod runner;
mod validators;

use runner::Runner;
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args();

    // TODO: add --eip flag to validate specific EIP
    if args.len() != 2 {
        println!("Usage: eipv [dir]");
        exit(1)
    }

    let dir = args.nth(1).unwrap();

    let mut runner = Runner::new(dir);
    runner.validate();

    println!("{}", runner);
}
