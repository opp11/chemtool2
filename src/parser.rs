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
//!    |  P
//! P --> EC
//!    |  E
//! E --> <text>
//!    |  (M)
//! C --> <number>

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

        // TODO: Make this cleaner
        if !self.eof() && (self.peek_char().is_alphabetic() || self.peek_char() == '(') {
            let mut molecule = try!(self.parse_molecule());
            out.extend(molecule.drain());
        }

        Ok(out)
    }

    fn parse_periodic(&mut self) -> CTResult<Vec<Token>> {
        let mut out = Vec::new();
        let mut elem = try!(self.parse_element());
        out.extend(elem.drain());

        if !self.eof() && self.peek_char().is_numeric() {
            let coef = try!(self.parse_coefficient());
            out.push(coef);
        }

        Ok(out)
    }

    fn parse_element(&mut self) -> CTResult<Vec<Token>> {
        if self.eof() {
            return Err(CTError {
                desc: "Found no periodic element".to_string(),
                pos: self.pos,
                len: 1,
            });
        }
        let start_pos = self.pos;
        let first = self.consume_char();
        if first == '(' {
            let mut out = Vec::new();
            out.push(Token { tok: ParenOpen, pos: start_pos, len: 1 });
            let mut molecule = try!(self.parse_molecule());
            out.extend(molecule.drain());

            if self.eof() || self.consume_char() != ')' {
                Err(CTError {
                    desc: "Missing closing parentheses".to_string(),
                    pos: self.pos - 1,
                    len: 1,
                })
            } else {
                out.push(Token { tok: ParenClose, pos: self.pos - 1, len: 1 });
                Ok(out)
            }
        } else if first.is_uppercase() {
            let mut name = String::new();
            name.push(first);
            name.push_str(self.consume_while(|ch| ch.is_lowercase()).as_slice());
            let len = name.len();
            Ok(vec!(Token{ tok: Elem(name), pos: start_pos, len: len }))
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

#[cfg(test)]
mod test {
    use super::*;
    use error::CTResult;
    use token::Token;
    use token::TokenKind::*;

    fn check_raw_result(raw_result: CTResult<Vec<Token>>, expected: Vec<Token>) {
        if let Ok(result) = raw_result {
            assert_eq!(result, expected);
        } else {
            panic!(raw_result);
        }
    }

    #[test]
    fn elems() {
        let mut parser = Parser::new("CHeH");
        let raw_result = parser.parse_molecule();
        let expected = vec!(Token { tok: Elem("C".to_string()), pos: 0, len: 1 },
                            Token { tok: Elem("He".to_string()), pos: 1, len: 2 },
                            Token { tok: Elem("H".to_string()), pos: 3, len: 1 });
        check_raw_result(raw_result, expected);
    }

    #[test]
    fn coefs() {
        let mut parser = Parser::new("C23");
        let raw_result = parser.parse_molecule();
        let expected = vec!(Token { tok: Elem("C".to_string()), pos: 0, len: 1 },
                            Token { tok: Coefficient(23), pos: 1, len: 2 });
        check_raw_result(raw_result, expected);
    }

    #[test]
    fn parens() {
        let mut parser = Parser::new("(C)2");
        let raw_result = parser.parse_molecule();
        let expected = vec!(Token { tok: ParenOpen, pos: 0, len: 1 },
                            Token { tok: Elem("C".to_string()), pos: 1, len: 1},
                            Token { tok: ParenClose, pos: 2, len: 1},
                            Token { tok: Coefficient(2), pos: 3, len: 1 });
        check_raw_result(raw_result, expected);
    }
}