use std::f64;
use std::cmp::min;
use std::num::Float;
use std::ops::{Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
use elem;
use elem::Molecule;
use error::{CTResult, CTError};
use error::CTErrorKind::InputError;

macro_rules! impl_matrix_index {
    ($idx:ty, $out:ty) => {
        impl Index<$idx> for Matrix {
            type Output = $out;

            fn index(&self, index: &$idx) -> &$out {
                &self.buf[*index]
            }
        }
    }
}

macro_rules! impl_matrix_index_mut {
    ($idx:ty, $out:ty) => {
        impl IndexMut<$idx> for Matrix {
            fn index_mut(&mut self, index: &$idx) -> &mut $out {
                &mut self.buf[*index]
            }
        }
    }
}

pub fn pretty_print_balanced(reaction: &(Vec<Molecule>, Vec<Molecule>), coefs: &Vec<u32>) {
    let &(ref lhs, ref rhs) = reaction;
    print!("{} {}", coefs[0], lhs[0]);
    for (coef, molecule) in coefs.iter().zip(lhs.iter()).skip(1) {
        print!(" + {} {}", coef, molecule);
    }
    print!(" -> ");
    print!("{} {}", coefs[lhs.len()], rhs[0]);
    for (coef, molecule) in coefs.iter().skip(lhs.len()).zip(rhs.iter()).skip(1) {
        print!(" + {} {}", coef, molecule);
    }
    println!("");
}

/// Balances a chemical reaction using Gaussian elimination
pub fn balance_reaction(reaction: &(Vec<Molecule>, Vec<Molecule>)) -> CTResult<Vec<u32>> {
    let reac_mat = Matrix::from_reaction(reaction);
    let reduced_mat = try!(forward_elim(reac_mat));
    let coefs = back_substitute(&reduced_mat);

    // if any of the coefs are 0, then an element in that molecule is missing on the other side
    // of the reaction
    if let Some(pos) = coefs.iter().position(|&c| c == 0.0) {
        let &(ref lhs, ref rhs) = reaction;
        let molecule = lhs.iter().chain(rhs.iter()).nth(pos).unwrap();
        let begin = molecule.first().unwrap().pos;
        let len = molecule.last().unwrap().pos + molecule.last().unwrap().len - begin;
        return Err(CTError {
            kind: InputError,
            desc: format!("An element in {} is missing on the other side of the reaction",
                          molecule),
            pos: Some((begin, len)),
        })
    }

    // we find the minimum element with fold, since f64 does not implement Ord...
    let min = coefs.iter().fold(f64::INFINITY, |crnt, &num| {
        if num < crnt {
            num
        } else {
            crnt
        }
    });
    // now we divide all elements by the minimum to (hopefully) convert them all to intergers
    // after which we do the actual conversion
    // we also reverse the Vec so the coefs are in the right order
    Ok(coefs.iter().rev().map(|n| (n / min) as u32).collect())
}

fn column_abs_max_index(columns: &[Vec<f64>], column: usize) -> usize {
    let mut max = 0;
    for i in 1..columns.len() {
        if columns[i][column].abs() > columns[max][column].abs() {
            max = i;
        }
    }
    max
}

fn forward_elim(mut mat: Matrix) -> CTResult<Matrix> {
    for k in 0..min(mat.width(), mat.height()) {
        // locate the pivot
        let pivot = column_abs_max_index(&mat[k..], k) + k;
        if mat[pivot][k] == 0_f64 {
            return Err(CTError {
                kind: InputError,
                desc: "Could not balance reaction".to_string(),
                pos: None,
            })
        }
        // move the pivot to its new position
        mat.switch_rows(k, pivot);

        // zero out the rest of the column
        for i in k + 1..mat.height() {
            let mult = -(mat[i][k] / mat[k][k]);
            mat.add_row_to_row(i, k, mult);
        }
    }
    Ok(mat)
}

fn back_substitute(mat: &Matrix) -> Vec<f64> {
    let mut vars = Vec::with_capacity(mat.width());
    for k in (0..mat.width()).rev() {
        if k >= mat.height() {
            // if this coef does not have a corresponding row in the matrix, then treat is as a
            // free variable and set it to 1
            vars.push(1_f64);
        } else {
            let mut var = 0f64;
            for i in k + 1..mat.width() {
                var -= mat[k][i] * vars[mat.width() - 1 - i];
            }
            vars.push(var / mat[k][k]);
        }
    }
    vars
}

#[derive(Debug, PartialEq)]
struct Matrix {
    buf: Vec<Vec<f64>>,
    height: usize,
    width: usize,
}

impl Matrix {
    fn from_reaction(reaction: &(Vec<Molecule>, Vec<Molecule>)) -> Matrix {
        let &(ref lhs, ref rhs) = reaction;
        let mut names = Vec::<String>::new();
        for molecule in lhs.iter().chain(rhs.iter()) {
            let grouped = elem::group_elems(molecule.clone());
            for elem in grouped.into_iter() {
                if names.iter().find(|e| **e == elem.name).is_none() {
                    names.push(elem.name);
                }
            }
        }
        let mut buf = Vec::with_capacity(names.len());
        for name in names.iter() {
            let mut row = Vec::with_capacity(lhs.len() + rhs.len());
            for molecule in lhs.iter() {
                row.push(molecule.iter()
                                 .find(|e| e.name == *name)
                                 .and_then(|e| Some(e.coef as f64))
                                 .unwrap_or(0_f64));
            }
            // we loop over rhs seperately, since we need to multiply the coefs with -1
            for molecule in rhs.iter() {
                row.push(molecule.iter()
                                 .find(|e| e.name == *name)
                                 .and_then(|e| Some(-1.0*(e.coef as f64)))
                                 .unwrap_or(0_f64));
            }
            buf.push(row);
        }
        let height = names.len();
        let width = lhs.len() + rhs.len();
        Matrix {
            buf: buf,
            height: height,
            width: width,
        }
    }

    fn add_row_to_row(&mut self, dest: usize, row: usize, mult: f64) {
        for i in 0..self.width {
            let incr = self.buf[row][i] * mult;
            self.buf[dest][i] += incr;
        }
    }

    fn switch_rows(&mut self, row1: usize, row2: usize)  {
        self.buf.swap(row1, row2);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl_matrix_index!(usize, Vec<f64>);
impl_matrix_index!(Range<usize>, [Vec<f64>]);
impl_matrix_index!(RangeTo<usize>, [Vec<f64>]);
impl_matrix_index!(RangeFrom<usize>, [Vec<f64>]);
impl_matrix_index!(RangeFull, [Vec<f64>]);

impl_matrix_index_mut!(usize, Vec<f64>);
impl_matrix_index_mut!(Range<usize>, [Vec<f64>]);
impl_matrix_index_mut!(RangeTo<usize>, [Vec<f64>]);
impl_matrix_index_mut!(RangeFrom<usize>, [Vec<f64>]);
impl_matrix_index_mut!(RangeFull, [Vec<f64>]);

#[cfg(test)]
mod test {
    use super::*;
    use elem::PerElem;
    use error::CTErrorKind::InputError;

    macro_rules! dummy_elem(
        ($name:expr) => (
            PerElem { name: $name.to_string(), coef: 1, pos: 0, len: 1 }
        );
        ($name:expr, $coef:expr) => (
            PerElem { name: $name.to_string(), coef: $coef, pos: 0, len: 1 }
        );
    );

    #[test]
    fn balance() {
        // attempt to balance C3H8 + O2 -> CO2 + H2O
        let reaction = (vec!(vec!(dummy_elem!("C", 3), dummy_elem!("H", 8)),
                             vec!(dummy_elem!("O", 2))),
                        vec!(vec!(dummy_elem!("C", 1), dummy_elem!("O", 2)),
                             vec!(dummy_elem!("H", 2), dummy_elem!("O", 1))));
        let result = balance_reaction(&reaction);
        let expected = Ok(vec!(1, 5, 3, 4));
        assert_eq!(result, expected);
    }

    #[test]
    fn no_balance_needed() {
        let reaction = (vec!(vec!(dummy_elem!("C", 1)), vec!(dummy_elem!("H", 1))),
                        vec!(vec!(dummy_elem!("C", 1)), vec!(dummy_elem!("H", 1))));
        let result = balance_reaction(&reaction);
        let expected = Ok(vec!(1, 1, 1, 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn missing_elem() {
        let reaction = (vec!(vec!(dummy_elem!("C", 1)), vec!(dummy_elem!("H", 1))),
                        vec!(vec!(dummy_elem!("C", 1))));
        let result = balance_reaction(&reaction);
        println!("{:?}", result);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind, InputError);
    }
}