use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let new_team = String::from("blue");

    let score = scores.get(&new_team).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} - {value}");
    }
}
