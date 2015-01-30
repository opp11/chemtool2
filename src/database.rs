use std::old_io::File;
use std::old_io::SeekStyle::SeekSet;
use std::old_io::IoErrorKind::EndOfFile;
use elem::{PerElem, Molecule};
use error::{CTError, CTResult};
use error::CTErrorKind::{SyntaxError, DatabaseError};

macro_rules! read_err (
    () => (Err(CTError {
        kind: DatabaseError,
        desc: "Error reading the database".to_string(),
        pos: None
    }));
);

#[derive(Show, PartialEq)]
pub struct ElemData {
    pub short_name: String,
    pub long_name: String,
    pub mass: f64,
    pub atomic_num: u16,
}

pub struct ElemDatabase {
    db: File,
}

impl ElemDatabase {
    pub fn open(path: &Path) -> CTResult<ElemDatabase> {
        match File::open(path) {
            Ok(db_file) => Ok(ElemDatabase { db: db_file }),
            Err(_) => Err(CTError {
                kind: DatabaseError,
                desc: format!("Could not open database file. Expected at: {:?}",
                              path.as_str().unwrap_or("same directory as the program")),
                pos: None,
            }),
        }
    }

    pub fn get_single_data(&mut self, elem: &PerElem) -> CTResult<ElemData> {
        // we return to the beginning of the underlying file, since the data
        // might lie on a line we have previously read past
        self.db.seek(0, SeekSet).ok().expect("Internal error reading database");

        loop {
            // TODO: make so this function returns the 'not found' error
            let line = try!(self.read_line(elem));
            if line.starts_with(elem.name.as_slice()) {
                return decode_line(&line);
            }
        }
    }

    pub fn get_data(&mut self, elems: &Molecule) -> CTResult<Vec<ElemData>> {
        let mut out = Vec::new();
        for elem in elems.iter() {
            match self.get_single_data(elem) {
                Ok(data) => out.push(data),
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }

    fn read_line(&mut self, elem: &PerElem) -> CTResult<String> {
        // we know that no line in the database is more than 30 characters long
        let mut buf = Vec::with_capacity(30);
        loop {
            match self.db.read_byte() {
                Ok(b) if b == b'\n' => break,
                Ok(b) => buf.push(b),
                // we let EOF error return without error, so we can check for it further up
                Err(ref e) if e.kind == EndOfFile => return Err(CTError {
                    kind: SyntaxError,
                    desc: format!("Could not find element: {:?}", elem.name),
                    pos: Some((elem.pos, elem.len)),
                }),
                Err(_) => return read_err!()
            }
        }
        String::from_utf8(buf).or_else(|_| read_err!())
    }
}

fn decode_line(line: &String) -> CTResult<ElemData> {
    let data: Vec<&str> = line.trim().split(';').collect();
    if data.len() < 4 {
        Err(CTError {
            kind: DatabaseError,
            desc: "Missing field in database".to_string(),
            pos: None
        })
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
            Err(CTError {
                kind: DatabaseError,
                desc: "Field in database corrupted".to_string(),
                pos: None,
            })
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::old_io::File;
    use std::old_io::fs;
    use elem::PerElem;

    fn make_dummy_db(name: &str, contents: &str) -> ElemDatabase {
        if let Err(e) = File::create(&Path::new(name)).and_then(|mut f| f.write_str(contents)) {
            // if we can't make the database we can't test, so just abort here
            panic!("Could not create dummy database: {:?}", e.desc);
        }
        ElemDatabase::open(&Path::new(name)).unwrap()
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
            B;3;Beta;4\n");
        let raw_result = db.get_data(&vec!(
            PerElem { name: "B".to_string(), coef: 1, pos: 0, len: 1 },
            PerElem { name: "A".to_string(), coef: 1, pos: 1, len: 1 }
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
            C;0;Coop;0\n");
        let raw_result = db.get_single_data(
            &PerElem { name: "B".to_string(), coef: 1, pos: 0, len: 2 }
        );
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
        let mut db = make_dummy_db(db_name, "A;123.456789;Abba;12\n");
        let result = db.get_single_data(
            &PerElem { name: "B".to_string(), coef: 1, pos: 0, len: 2 }
        );
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }

    #[test]
    fn missing_field() {
        let db_name = "missing_field_db";
        let mut db = make_dummy_db(db_name, "A;");
        let result = db.get_single_data(
            &PerElem { name: "A".to_string(), coef: 1, pos: 0, len: 2 }
        );
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }

    #[test]
    fn field_corrupted() {
        let db_name = "field_corrupted_db";
        let mut db = make_dummy_db(db_name, "A;not a number;Abba;12\n");
        let result = db.get_single_data(
            &PerElem { name: "A".to_string(), coef: 1, pos: 0, len: 2 }
        );
        remove_dummy_db(db_name);
        assert!(result.is_err());
    }
}