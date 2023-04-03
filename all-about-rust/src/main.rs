mod datatypes;
mod error_handling;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // print::run();
    // datatypes::vars::run();
    // types::run();
    // datatypes::strings::run();
    // vectors::run();
    // tuples::run();
    // conditionals::run();
    // arrays::run();
    // functions::run();
    // structs::run();
    // enums::run();
    // datatypes::hashmaps::run();
    // let plant = Asparagus {};
    // println!("I'm growing {:?}", plant);
    // error_handling::unrecoverable_errors::run();
    error_handling::recoverable_errors::run();
}
