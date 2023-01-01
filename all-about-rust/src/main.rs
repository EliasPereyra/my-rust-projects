mod datatypes;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // print::run();
    // datatypes::vars::run();
    // types::run();
    // strings::run();
    // vectors::run();
    // tuples::run();
    // conditionals::run();
    // arrays::run();
    // functions::run();
    // structs::run();
    // enums::run();
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
