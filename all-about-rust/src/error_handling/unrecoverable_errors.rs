// Unrecoverable errors

pub fn run() {
    // basic panic
    // panic!("Crash and burn!")

    // Going beyond the length of a vector will cause a panic
    let vec = [1, 2, 3];

    // In the terminal output, rust will tell you exactly where the problem is
    // Even more, you can write: RUST_BACKTRACE=1 cargo run for getting more info
    // It gives you a long list, very detailed information of how was the compilation executed
    // Note: the result of the backtrace may vary depending on your OS and rust version
    vec[10]; // <-- will panic
}
