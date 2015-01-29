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
    RightArrow,
}

impl TokenKind {
    pub fn elem(&self) -> Option<&String> {
        if let TokenKind::Elem(ref name) = *self {
            Some(name)
        } else {
            None
        }
    }

    pub fn coef(&self) -> Option<&u32> {
        if let TokenKind::Coefficient(ref coef) = *self {
            Some(coef)
        } else {
            None
        }
    }
}