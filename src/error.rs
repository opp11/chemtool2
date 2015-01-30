#[derive(Show, PartialEq)]
pub struct CTError {
    pub kind: CTErrorKind,
    pub desc: String,
    pub pos: Option<(usize, usize)>,
}

#[derive(Show, PartialEq)]
pub enum CTErrorKind {
    SyntaxError,
    DatabaseError,
}

pub type CTResult<T> = Result<T, CTError>;

impl CTError {
    pub fn print(&self, input: &str) {
        match self.kind {
            CTErrorKind::SyntaxError => {
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