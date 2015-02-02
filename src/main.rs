#![allow(unused_features)] // so we can still feature(os) when testing
#![feature(collections, path, io, core, os, plugin)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[plugin] #[no_link] extern crate docopt_macros;

use std::os;

mod elem;
mod parser;
mod error;
mod database;
mod mass;

docopt!(Args, "
usage:
    chemtool <formula> [options]
    chemtool [-h | --help]
    chemtool [-v | --version]

options:
    -h --help       Display this message and then exit.
    -v --version    Display the version number and then exit.
    --db-path PATH  Explicitly specify the path to the database file.
");

const VERSION: &'static str = "chemtool 0.2.0";

#[cfg(not(test))]
fn main() {
    let args: Args = Args::docopt().help(true)
                             .version(Some(VERSION.to_string()))
                             .decode()
                             .unwrap_or_else(|e| e.exit());
    let path = if !args.flag_db_path.is_empty() {
        Path::new(args.flag_db_path)
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