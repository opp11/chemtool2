#![allow(unused_features)] // so we can still feature(os) when testing
#![feature(collections, path, io, core, os, plugin, env)]
extern crate getopts;

use getopts::Options;
use std::env;
use parser::Parser;
use database::ElemDatabase;
use error::{CTResult, CTError};
use error::CTErrorKind::{InputError, UsageError};

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

const VERSION: &'static str = "chemtool 0.4.1";

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

        let cmd_result = if given_opts.free.len() > 0 {
            let cmd = &given_opts.free[0];
            let args = given_opts.free.tail();
            match cmd.as_slice() {
                "mass" => mass_cmd(&args, &path),
                "balance" => balance_cmd(&args),
                _ => {
                    Err(CTError {
                        kind: UsageError,
                        desc: "Invalid command".to_string(),
                        pos: None,
                    })
                }
            }
        } else {
            Err(CTError {
                kind: UsageError,
                desc: "Missing command.".to_string(),
                pos: None,
            })
        };

        match cmd_result {
            Err(ref e) if e.kind == InputError => e.print(Some(&args[2])),
            Err(ref e) if e.kind == UsageError => e.print(Some(&opts.usage(USAGE))),
            Err(ref e) => e.print(None),
            _ => ()
        }
    }
}

fn mass_cmd(args: &[String], db_path: &Path) -> CTResult<()> {
    if args.len() < 1 {
        Err(CTError {
            kind: UsageError,
            desc: "Missing formula.".to_string(),
            pos: None,
        })
    } else if args.len() > 1 {
        Err(CTError {
            kind: UsageError,
            desc: "Too many arguments.".to_string(),
            pos: None,
        })
    } else {
        let input = args[0].as_slice();
        let mut parser = Parser::new(input);
        let molecule = try!(parser.parse_molecule());
        if !parser.is_done() {
            // since there should be no whitespace in a molecule, the only way for parser to have
            // returned sucess while not being done, is if there was some whitespace,
            // followed by more (illegal) input
            return Err(CTError {
                kind: InputError,
                desc: "A molecule must not contain whitespace".to_string(),
                pos: None,
            })
        }

        let molecule = elem::group_elems(molecule);
        let mut database = try!(ElemDatabase::open(db_path));
        let data = try!(database.get_data(&molecule));
        mass::pretty_print_data(&data, &molecule);
        Ok(())
    }
}

fn balance_cmd(args: &[String]) -> CTResult<()> {
    if args.len() < 1 {
        Err(CTError {
            kind: UsageError,
            desc: "Missing reaction.".to_string(),
            pos: None,
        })
    } else if args.len() > 1 {
        Err(CTError {
            kind: UsageError,
            desc: "Too many arguments.".to_string(),
            pos: None,
        })
    } else {
        let input = args[0].as_slice();
        let mut parser = Parser::new(input);
        let reaction = try!(parser.parse_reaction());
        let coefs = try!(balance::balance_reaction(&reaction));
        balance::pretty_print_balanced(&reaction, &coefs);
        Ok(())
    }
}