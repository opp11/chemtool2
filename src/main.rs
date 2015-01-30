#![feature(collections, path, io, core)]

mod elem;
mod parser;
mod error;
mod database;
mod mass;

#[cfg(not(test))]
fn main() {
    let input = "CH3";
    if let Err(e) = mass::pretty_print_mass(input, "elemdb.csv") {
        println!("{:?}", e);
    }
}