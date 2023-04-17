// Generic Types
// Simple example. We want to loop through many vectors and find the largest number from the list.
// First off, before a generic type, we can extract the logic in another function and in that way
// we avoid duplication of code.

// In this case we extract the code in a separate function which receives a numbers list and find
// the largest number and returns it
/*
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num
        }
    }

    largest
}
*/

// The docs' definition for generics is well made:
// Quote: "We read this definition as: the function largest is generic over some type T.
// This function has one parameter named list, which is a slice of values of type T.
// The largest function will return a reference to a value of the same type T."
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // Note: std::cmp::PartialOrd allows comparsions on int and chars with an order.
    // Without this trait, the compile will throw an error, because it evaluates all types possible,
    // and a boolean or binary type doesn't have an order.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic in struct, thus says the docs: "this definition says that the Point<T>
// struct is generic over some type T, and the fields x and y are both that same type,
// whatever that type may be"
struct Point<T> {
    x: T,
    y: T,
}

// If I implement that struct like: let points = Point{x:5, y: 5.0} <-- this won't compile
// because the compiler gets the data type of the first field, and there's a mismatch.
// To solve this, and you really different values, you can use different generic types.

struct DifferentPoints<T, U> {
    x: T,
    y: U,
}

// Enums, as for structs, enums can operate over generic data types and its values too.
// Some holds a value of type T and None is optional
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Thus says the docs: "When you recognize situations in your code with multiple struct or
// enum definitions that differ only in the types of the values they hold, you can avoid
// duplication by using generic types instead."

pub fn run() {
    let numbers_list = vec![24, 56, 72, 80, 45];

    let result = largest(&numbers_list);
    println!("The largest number is: {}", result);

    let numbers_list = vec!['m', 'o', 'n', 't', 'h'];

    let result = largest(&numbers_list);
    println!("The largest char is: {}", result);

    let points = Point { x: 5, y: 6 };
    println!("The points entered are: x = {}, y = {}", points.x, points.y);

    let diff_points = DifferentPoints { x: 5, y: 5.5 };
    println!(
        "Now the points are different: x = {}, y = {}",
        diff_points.x, diff_points.y
    );
}
