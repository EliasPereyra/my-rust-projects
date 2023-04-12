// Generic Types
// Simple example. We want to loop through many vectors and find the largest number from the list.
// First off, before a generic type, we can extract the logic in another function and in that way
// we avoid duplication of code.

// In this case we extract the code in a separate function which receives a numbers list and find
// the largest number and returns it
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num
        }
    }

    largest
}

pub fn run() {
    let numbers_list = vec![24, 56, 72, 80, 45];

    let result = largest(&numbers_list);
    println!("The largest number is: {}", result);

    let numbers_list = vec![245, 562, 727, 805, 454];

    let result = largest(&numbers_list);
    println!("The largest number is: {}", result);
}
