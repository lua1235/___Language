use std::io;
use std::fs::File;
use std::env;

use ast::format::AstFormat;
use ast::walker::AstWalker;
use scanner::Scanner;
use parser::Parser;

mod ast;

mod scanner;
mod parser;
mod name_resolution;

fn main() -> io::Result<()> {
    let filepath = parse_args(env::args());
    let file = File::open(&filepath)?;
    let mut scanner = Scanner::new(file);
    /*
    for tok in scanner.collect::<Vec<_>>() {
        println!("{tok:?}");
    }
    let file2 = File::open(&filepath)?;
    scanner = Scanner::new(file2);
    */
    let mut parser = Parser::new();
    let ast = parser.gen_ast(&mut scanner);
    let mut ast_formatter = AstFormat::new(); 
    println!("{}", ast_formatter.walk(&ast));
    Ok(())
}

fn parse_args(mut args : impl Iterator<Item = String>) -> String {
    args.next();
    match args.next() {
        Some(arg) => arg,
        None => panic!("No filepath provided!")
    }
}
