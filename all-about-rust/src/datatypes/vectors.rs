use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];

    println!("The original vector: {:?}", nums);
    // Re-assign value
    nums[2] = 6;
    println!("Now the vector was modified at the position 2: {:?}", nums);

    // Add to the vector
    nums.push(5);
    println!("A new value was added: {:?}", nums);

    nums.pop();
    println!(
        "But with the pop function, we can remove the Last value: {:?}",
        nums
    );

    /*  Vectors are heaped allocated. The online book or the e-book says, quote:
        "Unlike the built-in array and tuple types, the data these collections point to
        is stored on the heap, which means the amount of data does not need to be known
        at compile time and can grow or shrink as the program runs."
    */
    println!(
        "Vectors are heaped allocated, they occupy {} bytes",
        mem::size_of_val(&nums)
    );

    // Slice in rust
    let slice = &nums[1..2];
    println!("Now printing a slice: {:?}", slice);

    // Loop through a vector
    for n in nums.iter() {
        println!("The number is: {}", n);
    }

    // And in a loop we can modify the values
    for n in nums.iter_mut() {
        *n *= 2;
        println!("{}", n);
    }
}
