#![allow(unstable)]

mod token;
mod parser;
mod error;

#[cfg(not(test))]
fn main() {
    let mut db = ElemDatabase::open("elemdb.csv").ok().unwrap();
    let tok = Token { tok: Elem("He".to_string()), pos: 0, len: 2 };
    println!("{:?}", db.get_single_data(&tok));
}