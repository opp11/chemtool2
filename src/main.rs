#![allow(unused_features)] // so we can still feature(os) when testing
#![feature(collections, path, io, core, os, plugin, env)]
extern crate getopts;

use getopts::Options;
use std::env;
use parser::Parser;

mod elem;
mod parser;
mod error;
mod database;
mod mass;
mod balance;

const USAGE: &'static str = "\
Usage:
    chemtool mass <formula> [options]
    chemtool balance <reaction> [options]
    chemtool [-h | --help]
    chemtool [-v | --version]";

const VERSION: &'static str = "chemtool 0.3.0";

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = env::args().collect();
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
            let cmd = &given_opts.free[0];
            let args = given_opts.free.tail();
            match cmd.as_slice() {
                "mass" => mass_cmd(&args, &opts, &path),
                "balance" => balance_cmd(&args, &opts),
                _ => {
                    println!("Invalid command");
                    println!("{}", opts.usage(USAGE));
                }
            }
        } else {
            println!("Missing command.");
            println!("{}", opts.usage(USAGE));
        }
    }
}

fn mass_cmd(args: &[String], opts: &Options, db_path: &Path) {
    if args.len() < 1 {
        println!("Missing formula.");
        println!("{}", opts.usage(USAGE));
    } else if args.len() > 1 {
        println!("Too many arguments.");
        println!("{}", opts.usage(USAGE));
    } else {
        let formula = args[0].as_slice();
        if let Err(e) = mass::pretty_print_mass(formula, &db_path) {
            e.print(formula);
        }
    }
}

fn balance_cmd(args: &[String], opts: &Options) {
    if args.len() < 1 {
        println!("Missing reaction.");
        println!("{}", opts.usage(USAGE));
    } else if args.len() > 1 {
        println!("Too many arguments.");
        println!("{}", opts.usage(USAGE));
    } else {
        let input = args[0].as_slice();
        let mut parser = Parser::new(input);
        let reaction = match parser.parse_reaction() {
            Ok(r) => r,
            Err(e) => {
                e.print(input);
                return;
            }
        };
        let coefs = match balance::balance_reaction(&reaction) {
            Ok(c) => c,
            Err(e) => {
                e.print(input);
                return;
            }
        };
        balance::pretty_print_balanced(&reaction, &coefs);
    }
}