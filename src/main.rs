#![allow(unstable)]

use parser::Parser;

mod token;
mod parser;
mod error;

fn main() {
    let mut parser = Parser::new("C3HeH4");
    println!("{:?}", parser.parse_molecule());
}