use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::env;

use scanner::Scanner;

mod common;
mod scanner;

fn main() -> io::Result<()> {
    let filepath = parse_args(env::args());
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut scanner = Scanner::new();
    scanner.scan(&mut reader.lines());
    for tok in scanner.get_token_iter() {
        println!("{tok:?}");
    }
    Ok(())
}

fn parse_args(mut args : impl Iterator<Item = String>) -> String {
    args.next();
    match args.next() {
        Some(arg) => arg,
        None => panic!("No filepath provided!")
    }
}
