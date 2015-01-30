#![feature(collections, path, io, core, os)]

use std::os;

mod elem;
mod parser;
mod error;
mod database;
mod mass;

#[cfg(not(test))]
fn main() {
    let input = "CH3";
    let mut path = Path::new(&os::args()[0]);
    path.set_filename("elemdb.csv");
    if let Err(e) = mass::pretty_print_mass(input, &path) {
        println!("{:?}", e);
    }
}