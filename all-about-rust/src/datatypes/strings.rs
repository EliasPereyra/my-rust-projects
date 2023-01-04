pub fn run() {
    let mut hello = String::from("Hello");

    println!("The length of hello is {}", hello.len());

    hello.push('!');

    println!("Hello is changed: {}", hello);
    println!("Now the length of hello is: {}", hello.len());

    hello.push_str(" World");
    println!(
        "Now I added a string instead of a single character: {}",
        hello
    );

    println!(
        "We can see the capacity of a string var in bytes: {}",
        hello.capacity()
    );

    hello.push_str(" And everyone!");
    println!(
        "Now we added a new string into the var, and the capacity also increased: {}",
        hello.capacity()
    );

    println!("We can split a phrase by using the separator that the String mod has: ");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Concatenation with the + operator
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // instead of concatenation like: s1 + " " + s2 + " " + s3
    // we can simplify it by using the format! macro
    let strings = format!("{s1}-{s2}-{s3}");

    // Create a str var by setting the capacity
    let mut stringed = String::with_capacity(10);
    stringed.push('a');
    stringed.push('b');

    println!("This string is limited: {}", stringed);

    // Assertion testing
    assert_eq!(2, stringed.len());

    // Iterating on a string is very different unlike in other languages.
    // Rust doesn't allow you to access for eg. the first character of a string[0] <- this syntax isn't valid on rust
    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
    for c in "בית".chars() {
        println!("{c}");
    }

    // and instead, here I choose the bytes function. It returns each byte of each letter in the string
    for c in "בית".bytes() {
        println!("{c}");
    }
}
