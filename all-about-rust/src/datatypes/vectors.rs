use std::mem;

pub fn run() {
    let mut nums = vec![1, 2, 3, 4];

    println!("The original vector: {:?}", nums);
    // Re-assign value
    nums[2] = 6;
    println!("Now the vector was modified at the position 2: {:?}", nums);

    let third = &nums[2];
    println!("The third element of the nums vector is {}", third);

    let fourth_element = nums.get(3);
    match fourth_element {
        Some(fourth_element) => println!("The fourth element is {fourth_element}"),
        None => println!("There is no fourth element"),
    }

    // If I'd want to access to a non-existent value within the vector using index, the compiler would panic.
    // But if I use the .get method trying to access to this non-existent value, the compiler None but would not panic.
    let nonexistent_value = &nums[90]; // this would panic
    let nonexistent_num = nums.get(99); // this wouldn't, it'd just return None

    // Add to the vector
    nums.push(5);
    println!("A new value was added: {:?}", nums);

    nums.pop();
    println!(
        "But with the pop function, we can remove the Last value: {:?}",
        nums
    );

    /*  Vectors are heaped allocated. The book online or the e-book says, quote:
        "Unlike the built-in array and tuple types, the data these collections point to
        is stored on the heap, which means the amount of data does not need to be known
        at compile time and can grow or shrink as the program runs."
        https://doc.rust-lang.org/book/ch08-00-common-collections.html
    */
    println!(
        "Vectors are heaped allocated, they occupy {} bytes",
        mem::size_of_val(&nums)
    );

    let newvec: Vec<i32> = Vec::new();

    // Slice in rust
    let slice = &nums[1..2];
    println!("Now printing a slice: {:?}", slice);

    // Loop through a vector
    // get immutable references
    for value in &nums {
        println!("{value}");
    }

    for n in nums.iter() {
        println!("The number is: {}", n);
    }

    // And in a loop we can modify the values
    // one way to iterate over mutable references
    for value in &mut nums {
        *value += 50;
    }
    // To change the value that the mutable reference refers to, we have to use the * dereference operator
    // to get to the value in i before we can use the += operator

    // another way to iterave over mutable references
    for n in nums.iter_mut() {
        *n *= 2;
        println!("{}", n);
    }
}
