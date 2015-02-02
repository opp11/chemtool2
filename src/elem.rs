#[derive(Show, PartialEq)]
pub struct PerElem {
    pub name: String,
    pub coef: u32,
    pub pos: usize,
    pub len: usize,
}

pub type Molecule = Vec<PerElem>;

/// Sorts the PerElems and groups those with the same name field.
///
/// Grouping of two (or more) PerElems means adding the coef field of the
/// duplicate to the one already found, and then throwing away the duplicate.
/// E.g. CH3CH3 would turn into C2H6.
pub fn group_elems(mut molecule: Molecule) -> Molecule {
    let mut out = Vec::<PerElem>::new();
    {
        // we open a scope, so slice borrow stops before the for-loop
        let mut slice = molecule.as_mut_slice();
        slice.sort_by(|a, b| a.name.cmp(&b.name));
    }
    // since the elements are now sorted, if the current elem does not match the
    // last element in out (i.e. what we previously pushed), then it won't match
    // anything in out
    for elem in molecule.into_iter() {
        if out.last().and_then(|e| Some(e.name == elem.name)).unwrap_or(false) {
            out.last_mut().unwrap().coef += elem.coef;
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