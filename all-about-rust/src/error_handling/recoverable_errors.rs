use std::fs::File;
use std::io::ErrorKind;

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

    // Managing different errors
    /* -- Here we use a lot of match. we can simplify this code with closures (unwrap_or_else)
    let random_text2 = File::open("./src/error_handling/second_text.txt");

    let random_file2 = match random_text2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/error_handling/second_text.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem when creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    */

    let text_path = "./src/error_handling/second_text_with_closure.txt";

    let random_text3 = File::open(&text_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&text_path).unwrap_or_else(|error| {
                panic!("There was a problem when creating the file: {:?}", error);
            })
        } else {
            panic!("Problem when opening the file: {:?}", error);
        }
    });

    // Here we  use a function that saves us space writing match many times.
    // We have two options: unwrap() and exepect(); unwrap gives us two results, an OK, and an automatic panic,
    // i.e., We don't have to call a panic! manually, but it does it for us.
    // expect gives us two results too, but also gives us the liberty of providing a customized message if there was
    // an error, and the message is passed as a parameter.
    let greeting_file =
        File::open("./src/error_handling/hello.txt").expect("This file doesn't exist");
}
