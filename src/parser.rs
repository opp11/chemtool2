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
use elem::{PerElem, Molecule};
use error::{CTResult, CTError};
use error::CTErrorKind::SyntaxError;

pub struct Parser {
    pos: usize,
    input: String,
    paren_level: u32,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser { pos: 0, input: String::from_str(input), paren_level: 0 }
    }

    pub fn parse_reaction(&mut self) -> CTResult<(Vec<Molecule>, Vec<Molecule>)> {
        let lhs = try!(self.parse_side());
        self.consume_whitespace();

        // we do not care if some of the consumes are not called, since this is
        // an error anyway, and will abort the parsing
        if self.pos + 2 >= self.input.len() || self.consume_char() != '-' ||
                                               self.consume_char() != '>' {
            return Err(CTError {
                kind: SyntaxError,
                desc: "Missing arrow (->) in chemical reaction".to_string(),
                pos: Some((self.pos - 2, 1))
            });
        }
        self.consume_whitespace();

        let rhs = try!(self.parse_side());

        Ok((lhs, rhs))
    }

    pub fn parse_side(&mut self) -> CTResult<Vec<Molecule>> {
        let mut out = Vec::new();
        let molecule = try!(self.parse_molecule());
        out.push(molecule);
        self.consume_whitespace();

        if !self.eof() && self.peek_char() == '+' {
            self.consume_char();
            self.consume_whitespace();
            let mut rest = try!(self.parse_side());
            out.append(&mut rest);
        }

        Ok(out)
    }

    pub fn parse_molecule(&mut self) -> CTResult<Molecule> {
        let mut out = Vec::new();
        let mut per = try!(self.parse_periodic());
        out.append(&mut per);

        // TODO: Make this cleaner
        if !self.eof() && (self.peek_char().is_alphabetic() || self.peek_char() == '(') {
            let mut molecule = try!(self.parse_molecule());
            out.append(&mut molecule);
        }
        if !self.eof() && self.peek_char() == ')' && self.paren_level == 0 {
            Err(CTError {
                kind: SyntaxError,
                desc: "Missing opening parentheses".to_string(),
                pos: Some((self.pos, 1))
            })
        } else {
            Ok(out)
        }
    }

    fn parse_periodic(&mut self) -> CTResult<Vec<PerElem>> {
        let mut elem = try!(self.parse_element());

        if !self.eof() && self.peek_char().is_numeric() {
            let coef = try!(self.parse_coefficient());
            for e in elem.iter_mut() {
                e.coef *= coef;
            }
        }

        Ok(elem)
    }

    fn parse_element(&mut self) -> CTResult<Vec<PerElem>> {
        if self.eof() {
            return Err(CTError {
                kind: SyntaxError,
                desc: "Found no periodic element".to_string(),
                pos: Some((self.pos, 1))
            });
        }
        let start_pos = self.pos;
        let first = self.consume_char();
        if first == '(' {
            self.paren_level += 1;
            let molecule = try!(self.parse_molecule());
            if self.eof() || self.consume_char() != ')' {
                Err(CTError {
                    kind: SyntaxError,
                    desc: "Missing closing parentheses".to_string(),
                    pos: Some((self.pos - 1, 1))
                })
            } else {
                self.paren_level -= 1;
                Ok(molecule)
            }
        } else if first.is_uppercase() {
            let mut name = String::new();
            name.push(first);
            name.push_str(self.consume_while(|ch| ch.is_lowercase()).as_slice());
            let len = name.len();
            Ok(vec!(PerElem { name: name, coef: 1, pos: start_pos, len: len }))
        } else {
            Err(CTError {
                kind: SyntaxError,
                desc: "Missing uppercase letter at the beginning of the element".to_string(),
                pos: Some((self.pos - 1, 1))
            })
        }
    }

    fn parse_coefficient(&mut self) -> CTResult<u32> {
        let start_pos = self.pos;
        let num_str = self.consume_while(|ch| ch.is_numeric());
        if let Some(num) = num_str.parse::<u32>() {
            Ok(num)
        } else {
            Err(CTError {
                kind: SyntaxError,
                desc: "Could not parse coefficient".to_string(),
                pos: Some((start_pos, num_str.len()))
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
    use elem::PerElem;

    macro_rules! check_raw_result(
        ($raw:expr, $expected:expr) => (
            if let Ok(result) = $raw {
                assert_eq!(result, $expected);
            } else {
                panic!($raw);
            }
        )
    );

    #[test]
    fn elems() {
        let mut parser = Parser::new("CHeH");
        let raw_result = parser.parse_molecule();
        let expected = vec!(PerElem { name: "C".to_string(), coef: 1, pos: 0, len: 1 },
                            PerElem { name: "He".to_string(), coef: 1, pos: 1, len: 2 },
                            PerElem { name: "H".to_string(), coef: 1, pos: 3, len: 1 });
        check_raw_result!(raw_result, expected);
    }

    #[test]
    fn coefs() {
        let mut parser = Parser::new("C23");
        let raw_result = parser.parse_molecule();
        let expected = vec!(PerElem { name: "C".to_string(), coef: 23, pos: 0, len: 1 });
        check_raw_result!(raw_result, expected);
    }

    #[test]
    fn parens() {
        let mut parser = Parser::new("(CH3)2");
        let raw_result = parser.parse_molecule();
        let expected = vec!(PerElem { name: "C".to_string(), coef: 2, pos: 1, len: 1 },
                            PerElem { name: "H".to_string(), coef: 6, pos: 2, len: 1 });
        check_raw_result!(raw_result, expected);
    }

    #[test]
    fn multiple_elems() {
        let mut parser = Parser::new("C + H");
        let raw_result = parser.parse_side();
        let expected = vec!(vec!(PerElem { name: "C".to_string(), coef: 1, pos: 0, len: 1 }),
                            vec!(PerElem { name: "H".to_string(), coef: 1, pos: 4, len: 1 }));
        check_raw_result!(raw_result, expected);
    }

    #[test]
    fn reaction() {
        let mut parser = Parser::new("C -> H");
        let raw_result = parser.parse_reaction();
        let expected = (vec!(vec!(PerElem { name: "C".to_string(), coef: 1, pos: 0, len: 1 })),
                        vec!(vec!(PerElem { name: "H".to_string(), coef: 1, pos: 5, len: 1 })));
        check_raw_result!(raw_result, expected);
    }

    #[test]
    fn empty() {
        let mut parser = Parser::new("");
        assert!(parser.parse_molecule().is_err());
        assert!(parser.parse_reaction().is_err());
    }

    #[test]
    fn no_uppercase() {
        let mut parser = Parser::new("c");
        assert!(parser.parse_molecule().is_err());
        assert!(parser.parse_reaction().is_err());
    }

    #[test]
    fn paren_error() {
        let mut parser = Parser::new("(C");
        assert!(parser.parse_molecule().is_err());
        assert!(parser.parse_reaction().is_err());
    }

    #[test]
    fn invald_num() {
        let mut parser = Parser::new("C999999999999999999999");
        assert!(parser.parse_molecule().is_err());
        assert!(parser.parse_reaction().is_err());
    }

    #[test]
    fn dangling_plus() {
        let mut parser = Parser::new("C + -> H");
        assert!(parser.parse_reaction().is_err());
    }
}