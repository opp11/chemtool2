//! Functions for parsing a molecule or a chemical reaction.
//! Mostly based on the HTML parser from the robinson browser engine:
//! https://github.com/mbrubeck/robinson
//!
//! The parser works with the following grammar:
//! R = (R)eaction
//! S = one (S)ide of a reaction
//! M = (M)olecule
//! P = Combination of a (P)eriodic element and maybe a coefficient
//! E = Periodic (E)lement
//! C = (C)oefficient
//!
//! R --> S -> S
//! S --> M + S
//!    |  M
//! M --> PM
//!    |  P(M)
//!    |  P
//! P --> EC
//!    |  E

use std::str::CharRange;
use token::Token;
use token::TokenKind::*;
use error::{CTResult, CTError};

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser { pos: 0, input: String::from_str(input) }
    }

    pub fn parse_molecule(&mut self) -> CTResult<Vec<Token>> {
        let mut out = Vec::new();
        let mut per = try!(self.parse_periodic());
        out.extend(per.drain());

        if !self.eof() {
            if self.peek_char() == '(' {
                out.push(Token { tok: ParenOpen, pos: self.pos, len: 1 });
                self.consume_char();
                let mut molecule = try!(self.parse_molecule());
                out.extend(molecule.drain());

                if self.eof() || self.consume_char() != ')' {
                    return Err(CTError {
                        desc: "Missing closing parentheses".to_string(),
                        pos: self.pos - 1,
                        len: 1,
                    });
                } else {
                    out.push(Token { tok: ParenClose, pos: self.pos - 1, len: 1 });
                }
            } else if self.peek_char().is_alphabetic() {
                let mut molecule = try!(self.parse_molecule());
                out.extend(molecule.drain());
            }
        }

        Ok(out)
    }

    fn parse_periodic(&mut self) -> CTResult<Vec<Token>> {
        let mut out = Vec::new();
        let elem = try!(self.parse_element());
        out.push(elem);

        if !self.eof() && self.peek_char().is_numeric() {
            let coef = try!(self.parse_coefficient());
            out.push(coef);
        }

        Ok(out)
    }

    fn parse_element(&mut self) -> CTResult<Token> {
        if self.eof() {
            return Err(CTError {
                desc: "Found no periodic element".to_string(),
                pos: self.pos,
                len: 1,
            });
        }
        let first = self.consume_char();
        if first.is_uppercase() {
            let mut name = String::new();
            let start_pos = self.pos;
            name.push(first);
            name.push_str(self.consume_while(|ch| ch.is_lowercase()).as_slice());
            let len = name.len();
            Ok(Token{ tok: Elem(name), pos: start_pos, len: len })
        } else {
            println!("{:?}", first);
            Err(CTError {
                desc: "Missing uppercase letter at the beginning of the element".to_string(),
                pos: self.pos - 1,
                len: 1,
            })
        }
    }

    fn parse_coefficient(&mut self) -> CTResult<Token> {
        let start_pos = self.pos;
        let num_str = self.consume_while(|ch| ch.is_numeric());
        if let Some(num) = num_str.parse::<u32>() {
            Ok(Token { tok: Coefficient(num), pos: start_pos, len: num_str.len() })
        } else {
            Err(CTError {
                desc: "Could not parse coefficient".to_string(),
                pos: start_pos,
                len: num_str.len(),
            })
        }
    }

    fn peek_char(&self) -> char {
        self.input.char_at(self.pos)
    }

    fn consume_char(&mut self) -> char {
        let CharRange { ch, next } = self.input.char_range_at(self.pos);
        self.pos = next;
        ch
    }

    fn consume_while<F>(&mut self, pred: F) -> String where F: Fn(char) -> bool {
        let mut out = String::new();
        while !self.eof() && pred(self.peek_char()) {
            out.push(self.consume_char());
        }
        out
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|ch| ch.is_whitespace());
    }

    fn eof(&mut self) -> bool {
        self.pos >= self.input.len()
    }
}