#![allow(unstable)]

use parser::Parser;

mod token;
mod parser;
mod error;

#[cfg(not(test))]
fn main() {
    let mut parser = Parser::new("C3HeH4");
    println!("{:?}", parser.parse_molecule());
}