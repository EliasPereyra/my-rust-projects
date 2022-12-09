pub fn run() {
    let mut hello = String::from("Hello");

    println!("The length of hello is {}", hello.len());

    hello.push('!');

    println!("Hello is changed: {}", hello);
    println!("Now the length of hello is: {}", hello.len());
}
