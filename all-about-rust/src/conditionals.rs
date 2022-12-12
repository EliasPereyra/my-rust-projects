pub fn run() {
    let person: (&str, &str, i8) = ("Elias", "Pereyra", 16);

    if person.0 != "" && person.1 != "" && person.2 > 18 {
        println!("Elias, you are already an adult. You can go on")
    } else if person.2 < 18 {
        println!("Wait, you cannot go in.")
    } else {
        println!("Show me your credentials to verify");
    }
}
