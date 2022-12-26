pub fn run() {
    // So a tuple is an array like, with fixed content and in types

    let person: (&str, &str, i8) = ("Elias", "Pereyra", 25);

    println!("So, I'm {} {}, I'm {}", person.0, person.1, person.2);
    println!("This was generated through a tuple, where I could mix strings and a number type. A tuple can be made of values with different types.");
}
