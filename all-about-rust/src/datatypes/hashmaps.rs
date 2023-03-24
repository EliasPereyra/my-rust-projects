use std::collections::HashMap;

// Hash Maps like vectors, are homogenous: both the keys and the values must be of the same type

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let new_team = String::from("blue");

    let score = scores.get(&new_team).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} - {value}");
    }

    // Hash Maps and Ownership

    let field_name = String::from("favorite food");
    let field_value = String::from("Pizza");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("Name: {}", field_name); <-- This won't work
    // the value of field_name is moved into map when the insert method occurs.
    // Both field vars cannot be called anymore, and now are part of the map var.

    // Updating a hash map
    scores.insert(String::from("Purple"), 10);
    scores.insert(String::from("Purple"), 20);
    println!("Purple was updated: {:?}", scores); // <-- This code will print 20.
                                                  // When updating a hash map, if the key has already a value, it can be replaced with the new value, but it cannot
                                                  // remain with the first value. In other words, it cannot contain two values (at least if they're combined), but
                                                  // it'll remain with the last value inserted

    // if the key doesn't exist, we have the entry method
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("Yellow inserted: {:?}", scores); // <-- This will print {blue: 10, yellow: 50, ...}
                                               // If blue already has a value, the or_insert method doesn't act, but if yellow doesn't exist,
                                               // the or_insert will add 50 as a value for the yellow key

    // Update a value based on the old value
    let text = "A wonderful world of the many wonderful worlds";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Times of word: {:?}", map2);
}
