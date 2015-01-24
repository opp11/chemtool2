#[derive(Show, PartialEq)]
pub struct Token {
    pub tok: TokenKind,
    pub pos: usize,
    pub len: usize,
}

#[derive(Show, PartialEq)]
pub enum TokenKind {
    Elem(String),
    Coefficient(u32),
    ParenOpen,
    ParenClose,
    Plus,
    LeftArrow,
}