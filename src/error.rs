#[derive(Debug, PartialEq)]
pub struct CTError {
    pub kind: CTErrorKind,
    pub desc: String,
    pub pos: Option<(usize, usize)>,
}

#[derive(Debug, PartialEq)]
pub enum CTErrorKind {
    InputError,
    DatabaseError,
}

pub type CTResult<T> = Result<T, CTError>;

impl CTError {
    /// Pretty-prints the CTError struct to stdout
    pub fn print(&self, input: &str) {
        match self.kind {
            CTErrorKind::InputError => {
                println!("error: {}", self.desc);
                if let Some((pos, len)) = self.pos {
                    println!("error:     {}", input);
                    print!("error:     ");
                    for _ in 0..pos {
                        print!(" ");
                    }
                    print!("^");
                    for _ in 1..len {
                        print!("~");
                    }
                    println!("");
                }
            },
            CTErrorKind::DatabaseError => {
                println!("fatal error: {}", self.desc);
            }
        }
    }
}