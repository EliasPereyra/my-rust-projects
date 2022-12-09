/*
Primitive Types
    Integers: u8, i8, u16, u32, i32, i64, u128, i128 <-- These refer to the number of bits that take in memory
    Floats: f32, f64;
    Boolean: true | false;
    Characters: char;
    Tuples: a type of array, but different to the array itself, regarding how it works;
    Arrays: these arrays are fixed in length and cannot be changed;
*/

pub fn run() {
    println!("Rust is a statically typed language. This means that rust must know what types are we using of all variables, at compile time.");
    println!("Although, the compiler is able to infer what type we want to use, based on the value we use and how we use it");

    let i = 8; // <-- Here, rust infers the type, in this case: i32, which is by default
    let y = 1.56; // <-- Here, we have a decimal type value, and for decimal types, i64 is set by default

    let z: i64 = 145200985; // <-- Here, we have stated explicitly the type we want to use

    println!("If you wonder, what would be the maximum value for i32 types or i64 ones? Well...");
    println!("The maximum for i32: {}", std::i32::MAX);
    println!(
        "The maximum for i64: {}. This one is crazy, isn't it? Really big one.",
        std::i64::MAX
    );

    let a = true;
    let b = false;
    println!(
        "About booleans? You know: or something is {} or something is {}. Just that simple.",
        a, b
    );

    let c = a == a;
    let d = b == b;
    println!("Also, to get boolean values we can use expressions that evaluate something and gives us back a response, but of boolean type. For eg.");
    println!("c evalutes: a: {0} == a: {0} is {1}", a, c);
    println!("d evaluates: b: {0} == b: {0} is {1}", b, d);
    println!("But a == b is {}", a == b);

    let categ = 'A';
    let categ = 'B';
}
