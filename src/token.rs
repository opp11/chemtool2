#[derive(Show)]
pub struct Token {
    pub tok: TokenKind,
    pub pos: usize,
    pub len: usize,
}

#[derive(Show)]
pub enum TokenKind {
    Elem(String),
    Coefficient(u32),
    ParenOpen,
    ParenClose,
}