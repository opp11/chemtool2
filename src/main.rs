#![allow(unused_features)] // so we can still feature(os) when testing
#![feature(collections, path, io, core, os)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use std::os;
use docopt::Docopt;

mod elem;
mod parser;
mod error;
mod database;
mod mass;

const USAGE: &'static str = "
usage: chemtool <formula>
       chemtool [-h | --help]

options:
    -h --help    Display this message and then exit.
";

#[derive(RustcDecodable)]
struct Args {
    arg_formula: String,
}

#[cfg(not(test))]
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let mut path = Path::new(&os::args()[0]);
    path.set_filename("elemdb.csv");
    if let Err(e) = mass::pretty_print_mass(args.arg_formula.as_slice(), &path) {
        println!("{:?}", e);
    }
}