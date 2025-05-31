use std::fs::File;
use std::io;
use std::io::BufReader;
use std::env;

mod common;
mod scanner;

fn main() -> io::Result<()> {
    let filepath = parse_args(env::args());
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    Ok(())
}

fn parse_args(mut args : impl Iterator<Item = String>) -> String {
    args.next();
    match args.next() {
        Some(arg) => arg,
        None => panic!("No filepath provided!")
    }
}
