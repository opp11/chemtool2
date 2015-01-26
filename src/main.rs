#![allow(unstable)]

use parser::Parser;
use database::{ElemDatabase, ElemData};
use token::Token;
use token::TokenKind::Elem;

mod token;
mod parser;
mod error;
mod database;

#[cfg(not(test))]
fn main() {
    let mut db = ElemDatabase::open("elemdb.csv").ok().unwrap();
    let tok = Token { tok: Elem("He".to_string()), pos: 0, len: 2 };
    println!("{:?}", db.get_elem_data(&tok));
}