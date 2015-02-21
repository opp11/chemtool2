#![allow(unused_features)] // so we can still feature(os) when testing
#![feature(collections, path, io, core, os, plugin)]
extern crate getopts;

use getopts::Options;
use std::os;

mod elem;
mod parser;
mod error;
mod database;
mod mass;
mod balance;

const USAGE: &'static str = "\
Usage:
    chemtool <formula>... [options]
    chemtool [-h | --help]
    chemtool [-v | --version]";

const VERSION: &'static str = "chemtool 0.2.0";

#[cfg(not(test))]
fn main() {
    let args = os::args();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this message and then exit.");
    opts.optflag("v", "version", "Display the version number and then exit.");
    opts.optopt("", "db-path", "Explicitly specify the path to the database file.", "PATH");
    let given_opts = match opts.parse(args.tail()) {
        Ok(go) => go,
        Err(msg) => {
            println!("{}", msg.to_string());
            println!("{}", opts.usage(USAGE));
            return;
        },
    };
    if given_opts.opt_present("help") {
        println!("{}", opts.usage(USAGE));
    } else if given_opts.opt_present("version") {
        println!("{}", VERSION);
    } else {
        let path = if let Some(path) = given_opts.opt_str("db-path") {
            Path::new(path)
        } else {
            let mut path = Path::new(&args[0]);
            path.set_filename("elemdb.csv");
            path
        };
        if given_opts.free.len() > 0 {
            let input = given_opts.free[0].as_slice();
            if let Err(e) = mass::pretty_print_mass(input, &path) {
                e.print(input);
            }
        } else {
            println!("Missing formula to parse.");
            println!("{}", opts.usage(USAGE));
        }
    }
}