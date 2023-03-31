use std::fs::File;

pub fn run() {
    // sometimes when handling errors, we don't want the program to stop because of an error.
    // Instead, we'd find an alternative and not stopping the program. For e.g. if a file doesn't exist
    // we just create the file instead of stopping the process
    let random_text = File::open("./src/error_handling/text.txt");

    // Recall: the Result type is an enum. It has two generic type parameters. One for success and the other for failure.
    let random_file = match random_text {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening this file: {:?}", error),
    };

    // The return type of File::open is a Result<T, E>
}
