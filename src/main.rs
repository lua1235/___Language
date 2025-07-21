use std::io;
use std::fs::File;
use std::env;

use scanner::Scanner;
use parser::Parser;

mod scanner;
mod parser;

fn main() -> io::Result<()> {
    let filepath = parse_args(env::args());
    let file = File::open(&filepath)?;
    let file2 = File::open(&filepath)?;
    let mut scanner = Scanner::new(file);
    for tok in scanner.collect::<Vec<_>>() {
        println!("{tok:?}");
    }
    scanner = Scanner::new(file2);
    let mut parser = Parser::new();
    parser.gen_ast(&mut scanner);
    parser.print_ast();
    Ok(())
}

fn parse_args(mut args : impl Iterator<Item = String>) -> String {
    args.next();
    match args.next() {
        Some(arg) => arg,
        None => panic!("No filepath provided!")
    }
}
