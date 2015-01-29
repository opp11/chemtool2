#[derive(Show, PartialEq)]
pub struct PerElem {
    pub name: String,
    pub coef: u32,
    pub pos: usize,
    pub len: usize,
}

pub type Molecule = Vec<PerElem>;