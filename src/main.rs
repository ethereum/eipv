mod eip;
mod validators;

use eip::Eip;

use anyhow::Error;
use std::env::args;
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

    let ret = validate(dir);
    println!("ret: {:?}", ret);
}

fn validate(dir: String) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        println!("{:?}", entry);
        if let Ok(entry) = entry {
            let eip: Result<Eip, Vec<Error>> = fs::read_to_string(entry.path()).unwrap().parse();

            println!("{:?}", eip);

            return Ok(());
        }
    }

    return Ok(());
}
