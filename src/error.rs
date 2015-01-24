#[derive(Show, PartialEq)]
pub struct CTError {
    pub desc: String,
    pub pos: usize,
    pub len: usize,
}

pub type CTResult<T> = Result<T, CTError>;