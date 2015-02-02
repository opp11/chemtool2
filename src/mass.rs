use elem;
use error::{CTResult, CTError};
use error::CTErrorKind::InputError;
use parser::Parser;
use database::ElemDatabase;

/// Takes a checmical formula containing a single molecule, and pretty print the mass
///
/// The function will print the molar mass (and some other data) for each element
/// in the given molecule, as well as the total molar mass. Note, this function
/// errors if the input is invalid, or it contains more than a single molecule.
pub fn pretty_print_mass(formula: &str, db_path: &Path) -> CTResult<()> {
    let mut parser = Parser::new(formula);
    let molecule = try!(parser.parse_molecule());
    if !parser.is_done() {
        // since there should be no whitespace in a molecule, the only way for parser to have
        // returned sucess while not being done, is if there was some whitespace,
        // followed by more (illegal) input
        return Err(CTError {
            kind: InputError,
            desc: "A molecule must not contain whitespace".to_string(),
            pos: None,
        })
    }

    let molecule = elem::group_elems(molecule);
    let mut db = try!(ElemDatabase::open(db_path));
    let elem_data = try!(db.get_data(&molecule));
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
    Ok(())
}