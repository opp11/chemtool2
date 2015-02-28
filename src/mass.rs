use elem::Molecule;
use database::ElemData;

/// Takes a parsed checmical formula containing a single molecule, and pretty print the mass
///
/// The function will print the molar mass (and some other data) for each element
/// in the given molecule, as well as the total molar mass.
pub fn pretty_print_data(elem_data: &Vec<ElemData>, molecule: &Molecule) {
    let total = elem_data.iter()
                         .zip(molecule.iter())
                         .fold(0f64, |t, (ref data, ref elem)| t + data.mass * elem.coef as f64);

    println!("abbrv.     amt.          M             name          Z");
    println!("------------------------------------------------------");
    for (data, elem) in elem_data.iter().zip(molecule.iter()) {
        println!("{: <3}  {: >10}    {: >12}    {: ^12}    {: >3}",
                 data.short_name,
                 elem.coef,
                 // extra format, since println! does not right-align the number
                 // when we specify the precision
                 format!("{:3.8}", data.mass),
                 data.long_name,
                 data.atomic_num);
    }
    println!("Total: {}", total);
}