use std::io::{File, BufferedReader};
use std::io::SeekStyle::SeekSet;
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
        // we return to the beginning of the underlying file, since the data
        // might lie on a line we have previously read past
        self.db.get_mut().seek(0, SeekSet).ok().expect("Internal error reading database");
        let mut line_iter = self.db.lines();
        let short_name = match elem.tok {
            Elem(ref name) => name.as_slice(),
            _ => unreachable!(),
        };
        match line_iter.find(|l| l.is_err() || l.as_ref().ok().unwrap().starts_with(short_name)) {
            Some(Ok(ref line)) => decode_line(line),
            Some(Err(_)) => Err(DatabaseError(CTDatabaseError {
                desc: "Error reading the database".to_string()
            })),
            None => Err(SyntaxError(CTSyntaxError {
                desc: format!("Could not find element: {:?}", short_name),
                pos: elem.pos,
                len: elem.len,
            })),
        }
    }

    pub fn get_data(&mut self, elems: &Vec<Token>) -> CTResult<Vec<ElemData>> {
        let mut out = Vec::new();
        for elem in elems.iter() {
           match self.get_single_data(elem) {
                Ok(data) => out.push(data),
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}

fn decode_line(line: &String) -> CTResult<ElemData> {
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
    use std::io::File;
    use std::io::fs;
    use token::Token;
    use token::TokenKind::Elem;

    fn make_dummy_db(name: &str, contents: &str) -> ElemDatabase {
        if let Err(e) = File::create(&Path::new(name)).and_then(|mut f| f.write_str(contents)) {
            // if we can't make the database we can't test, so just abort here
            panic!("Could not create dummy database: {:?}", e.desc);
        }
        ElemDatabase::open(name).unwrap()
    }

    fn remove_dummy_db(name: &str) {
        if let Err(e) = fs::unlink(&Path::new(name)) {
            // if we can't remove the database something is wrong, and we abort the test
            panic!("Could not remove dummy database: {:?}", e.desc);
        }
    }

    #[test]
    fn multiple_elems() {
        let db_name = "multiple_elems_db";
        let mut db = make_dummy_db(db_name,
            "A;1;Abba;2\n\
            B;3;Beta;4");
        let raw_result = db.get_data(&vec!(
            Token { tok: Elem("B".to_string()), pos: 0, len: 1 },
            Token { tok: Elem("A".to_string()), pos: 1, len: 1 }
        ));
        let expected = vec!(
            ElemData {
                short_name: "B".to_string(),
                long_name: "Beta".to_string(),
                mass: 3.0,
                atomic_num: 4,
            },
            ElemData {
                short_name: "A".to_string(),
                long_name: "Abba".to_string(),
                mass: 1.0,
                atomic_num: 2,
            }
        );
        remove_dummy_db(db_name);
        assert_eq!(Ok(expected), raw_result);
    }

    #[test]
    fn find_elem() {
        let db_name = "find_elem_db";
        let mut db = make_dummy_db(db_name,
            "A;0;Abba;0\n\
            B;123.456789;Beta;12\n\
            C;0;Coop;0");
        let raw_result = db.get_single_data(&Token { tok: Elem("B".to_string()), pos: 0, len: 2 });
        let expected = ElemData {
            short_name: "B".to_string(),
            long_name: "Beta".to_string(),
            mass: 123.456789,
            atomic_num: 12,
        };
        remove_dummy_db(db_name);
        assert_eq!(Ok(expected), raw_result);
    }

    #[test]
    fn missing_elem() {
        let db_name = "missing_elem_db";
        let mut db = make_dummy_db(db_name, "A;123.456789;Abba;12");
        let result = db.get_single_data(&Token { tok: Elem("B".to_string()), pos: 0, len: 2 });
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }

    #[test]
    fn decode() {
        let db_name = "decode_db";
        let mut db = make_dummy_db(db_name, "A;123.456789;Abba;12");
        let raw_result = db.get_single_data(&Token { tok: Elem("A".to_string()), pos: 0, len: 2 });
        let expected = ElemData {
            short_name: "A".to_string(),
            long_name: "Abba".to_string(),
            mass: 123.456789,
            atomic_num: 12,
        };
        remove_dummy_db(db_name);
        assert_eq!(Ok(expected), raw_result);
    }

    #[test]
    fn missing_field() {
        let db_name = "missing_field_db";
        let mut db = make_dummy_db(db_name, "A;");
        let result = db.get_single_data(&Token { tok: Elem("A".to_string()), pos: 0, len: 2 });
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }

    #[test]
    fn field_corrupted() {
        let db_name = "field_corrupted_db";
        let mut db = make_dummy_db(db_name, "A;not a number;Abba;12");
        let result = db.get_single_data(&Token { tok: Elem("A".to_string()), pos: 0, len: 2 });
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }
}