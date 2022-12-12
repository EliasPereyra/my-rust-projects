pub fn run() {
    let person: (&str, &str, i8) = ("", "Pereyra", 16);

    if person.0 != "" && person.1 != "" && person.2 > 18 {
        println!("Elias, you are already an adult. You can go on")
    } else if person.2 < 18 {
        println!("Wait, you cannot go in.")
    } else {
        println!("Show me your credentials to verify");
    }

    // shorthand for if statements
    let verify_name = if person.0 == "" { false } else { true };
    println!("Does the person have a name? {}", verify_name);
}
