use std::io::{File, BufferedReader};
use error::{CTResult, CTDatabaseError, CTSyntaxError};
use error::CTError::{DatabaseError, SyntaxError};
use token::Token;
use token::TokenKind::Elem;

#[derive(Show, PartialEq)]
pub struct ElemData {
    short_name: String,
    long_name: String,
    mass: f64,
    atomic_num: u16,
}

pub struct ElemDatabase {
    db: BufferedReader<File>,
}

impl ElemDatabase {
    pub fn open(path: &str) -> CTResult<ElemDatabase> {
        match File::open(&Path::new(path)) {
            Ok(db_file) => Ok(ElemDatabase { db: BufferedReader::new(db_file) }),
            Err(_) => Err(DatabaseError(CTDatabaseError {
                desc: format!("Could not open database file. Expected at: {:?}", path)
            })),
        }
    }

    pub fn get_single_data(&mut self, elem: &Token) -> CTResult<ElemData> {
        let mut line_iter = self.db.lines();
        let short_name = match elem.tok {
            Elem(ref name) => name.as_slice(),
            _ => unreachable!(),
        };
        // TODO: Make this cleaner, and avoid returns
        loop {
            match line_iter.next() {
                Some(Ok(ref line)) if line.starts_with(short_name) => return decode_line(line),
                Some(Err(_)) => return Err(DatabaseError(CTDatabaseError {
                    desc: "Error reading the database".to_string()
                })),
                None => return Err(SyntaxError(CTSyntaxError {
                    desc: format!("Could not find element: {:?}", short_name),
                    pos: elem.pos,
                    len: elem.len,
                })),
                _ => (),
            }
        }
    }
}

pub fn decode_line(line: &String) -> CTResult<ElemData> {
    let data: Vec<&str> = line.trim().split(';').collect();
    if data.len() < 4 {
        Err(DatabaseError(CTDatabaseError {
            desc: "Missing field in database".to_string()
        }))
    } else {
        let mass = data[1].parse::<f64>();
        let atomic_num = data[3].parse::<u16>();
        if let (Some(m), Some(an)) = (mass, atomic_num) {
            Ok(ElemData {
                short_name: data[0].to_string(),
                long_name: data[2].to_string(),
                mass: m,
                atomic_num: an,
            })
        } else {
            Err(DatabaseError(CTDatabaseError {
                desc: "Field in database corrupted".to_string()
            }))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode() {
        let raw_result = decode_line(&"A;123.456789;Abba;12".to_string());
        let expected = ElemData {
            short_name: "A".to_string(),
            long_name: "Abba".to_string(),
            mass: 123.456789,
            atomic_num: 12,
        };
        assert_eq!(Ok(expected), raw_result);
    }

    #[test]
    fn missing_field() {
        assert!(decode_line(&"A;".to_string()).is_err());
    }

    #[test]
    fn field_corrupted() {
        assert!(decode_line(&"A;not a number;Abba;12".to_string()).is_err());
    }
}