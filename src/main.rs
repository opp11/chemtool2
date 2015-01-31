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
usage:
    chemtool <formula> [options]
    chemtool [-h | --help]
    chemtool [-v | --version]

options:
    -h --help       Display this message and then exit.
    -v --version    Display the version number and then exit.
    --db-path PATH  Explicitly specify the path to the database file.
";

const VERSION: &'static str = "chemtool 0.1.1";

#[derive(RustcDecodable)]
struct Args {
    arg_formula: String,
    flag_db_path: Option<String>,
}

#[cfg(not(test))]
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.help(true)
                                 .version(Some(VERSION.to_string()))
                                 .decode()
                            })
                            .unwrap_or_else(|e| e.exit());
    let path = if let Some(path) = args.flag_db_path {
        Path::new(path)
    } else {
        let mut path = Path::new(&os::args()[0]);
        path.set_filename("elemdb.csv");
        path
    };
    let input = args.arg_formula.as_slice();
    if let Err(e) = mass::pretty_print_mass(input, &path) {
        e.print(input);
    }
}