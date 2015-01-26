#[derive(Show, PartialEq)]
pub enum CTError {
    SyntaxError(CTSyntaxError),
    DatabaseError(CTDatabaseError),
}

#[derive(Show, PartialEq)]
pub struct CTSyntaxError {
    pub desc: String,
    pub pos: usize,
    pub len: usize,
}

#[derive(Show, PartialEq)]
pub struct CTDatabaseError {
    pub desc: String,
}

pub type CTResult<T> = Result<T, CTError>;