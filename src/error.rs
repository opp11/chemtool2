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
    pub fn print(&self, extra_desc: Option<&String>) {
        match self.kind {
            CTErrorKind::InputError => {
                println!("{}", self.desc);
                if let (Some((pos, len)), Some(input)) = (self.pos, extra_desc) {
                    println!("    {}", input);
                    print!("    ");
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
                println!("{}", self.desc);
            }
        }
    }
}