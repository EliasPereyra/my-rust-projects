// What is a function? A snippet, a small bloack of code that performs opeartions and returns back a result - be it complete or not

pub fn run() {
    greeting("elias", "pereyra");

    let sum = add(2, 3);
    let sub = substract(10, 22);
    let mult = multiply(2.1, 22.39);
    let div = divide(10, 10);

    // Closure
    let total = |n1: i32, n2: i32| n1 + n2 + sum;

    println!("The sum is {}", total(2, 3));
    println!("The substract is {}", sub);
    println!("The multiply is {}", mult);
    println!("The division is {}", div);
}

fn greeting(name: &str, lastname: &str) {
    println!("My name is {} {}", name, lastname);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn substract(n1: i16, n2: i16) -> i16 {
    return n1 - n2;
}

fn multiply(n1: f32, n2: f32) -> f32 {
    n1 * n2
}

fn divide(n1: i8, n2: i8) -> i8 {
    n1 / n2
}
