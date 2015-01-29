#[derive(Show, PartialEq)]
pub struct PerElem {
    pub name: String,
    pub coef: u32,
    pub pos: usize,
    pub len: usize,
}

pub type Molecule = Vec<PerElem>;

pub fn group_elems(molecule: Molecule) -> Molecule {
    let mut out = Vec::<PerElem>::new();
    for elem in molecule.into_iter() {
        if let Some(pos) = out.iter().position(|e| e.name == elem.name) {
            out[pos].coef += elem.coef;
        } else {
            out.push(elem);
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! dummy_elem(
        ($name:expr) => (
            PerElem { name: $name.to_string(), coef: 1, pos: 0, len: 1 }
        );
        ($name:expr, $coef:expr) => (
            PerElem { name: $name.to_string(), coef: $coef, pos: 0, len: 1 }
        );
    );

    #[test]
    fn group() {
        let result = group_elems(vec!(dummy_elem!("C"), dummy_elem!("H"), dummy_elem!("C")));
        let expected = vec!(dummy_elem!("C", 2), dummy_elem!("H", 1));
        assert_eq!(result, expected);
    }
}