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
    UsageError,
}

pub type CTResult<T> = Result<T, CTError>;

impl CTError {
    /// Pretty-prints the CTError struct to stdout
    pub fn print(&self, extra_desc: Option<&String>) {
        println!("{}", self.desc);
        // some errors will have extra stuff to report to make the message clearer for the user
        match self.kind {
            CTErrorKind::InputError => {
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
            _ => (),
        }
    }
}