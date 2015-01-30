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