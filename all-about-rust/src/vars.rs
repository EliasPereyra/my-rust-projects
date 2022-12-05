pub fn run() {
    println!("This module is about vars");

    println!(
        "{0} hold primitve {1} or references to {0}",
        "Variables", "data"
    );
    println!("{} are immutable by default", "Variables");
    println!("Rust is a block-scoped language - this means that the code inside the curly braces is where only it will be executed and not outside.");
    let name = "Elias";

    println!("In this case, {name} was created with let. So let is a way to define variables or binding names, another name.");
    let mut age = 25;
    println!("Now, Rust tells you that defining variables are immutable by default, which means you cannot change the value. Like this variable age with the value {}. So the age is a variable that might change in the future, so in order to allow to update the value of this variable, Rust provides us with the mut keyword, and mut stands for mutable, that is, your var becomes mutable and can be changed or modified.", age);
    age = 26;
    println!(
        "Now you can see that the variable was modified with this value, {}",
        age
    );

    const ID: i32 = 00001;
    println!("Here, I used 'const' for the ID variable with a i32 type and the value is {ID}");

    let (my_name, my_age) = (name, age);
    println!("Rust allows us to use multiple variables, so now I can reuse my variables that I made above and assign them new names, so right now {my_name} is my_name and {my_age} is my_age.");
}
