use elem;
use error::CTResult;
use parser::Parser;
use database::ElemDatabase;

pub fn pretty_print_mass(formula: &str, db_path: &str) -> CTResult<()> {
    let molecule = try!(Parser::new(formula).parse_molecule());
    let molecule = elem::group_elems(molecule);
    let mut db = try!(ElemDatabase::open(db_path));
    let elem_data = try!(db.get_data(&molecule));
    let total = elem_data.iter()
                         .zip(molecule.iter())
                         .fold(0f64, |t, (ref data, ref elem)| t + data.mass * elem.coef as f64);

    for (data, elem) in elem_data.iter().zip(molecule.iter()) {
        println!("{: <3}  {: >5}    {: >14}    {: ^12}    {: >3}",
                 data.short_name,
                 elem.coef,
                 // extra format, since println! does not right-align the number
                 // when we specify the precision
                 format!("{:3.10}", data.mass),
                 data.long_name,
                 data.atomic_num);
    }
    println!("Total: {}", total);
    Ok(())
}