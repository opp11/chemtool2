CHEMTOOL
========
A commandline utility for making chemistry a little less tedious.

So far the program is able to calculate the molar mass for a given molecule,
or balance a given chemical reaction.

Examples:
---------
To get the total molar mass of the molecule `CH3CH2CH3` simply call chemtool as:
```
chemtool mass CH3CH2CH3
```
Which should generate the following output:
```
abbrv.     amt.          M             name          Z
------------------------------------------------------
C             3     12.01070000       Carbon         6
H             8      1.00794000      Hydrogen        1
Total: 44.09562
```
To balance a reaction - e.g. `C3H8 + O2 -> CO2 + H2O` call:
```
chemtool balance 'C3H8 + O2 -> CO2 + H2O'
```
and the following should be displayed:
```
1 C3H8 + 5 O2 -> 3 CO2 + 4 H2O
```

USAGE
=====
```
Usage:
    chemtool mass <formula> [options]
    chemtool balance <reaction> [options]
    chemtool [-h | --help]
    chemtool [-v | --version]

Options:
    -h --help           Display this message and then exit.
    -v --version        Display the version number and then exit.
    --db-path PATH      Explicitly specify the path to the database file.
```

Installing and building
=======================
The easiest way to compile the project is with the `cargo` program available
from the rust website at:
http://www.rust-lang.org/

Navigate to the project directory and call:
```
cargo build --release
```
which will place the binary in the target directory. Then copy the element
database `elemdb.csv` to the same directory as the program binary (or use
`--db-path` when invoking chemtool to specify the path yourself).

Testing
-------
To do a quick test of the program simply call:
```
cargo test
```