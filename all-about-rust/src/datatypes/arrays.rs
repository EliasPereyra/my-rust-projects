use std::mem;

// Arrays: is a fixed data structure, where 'fixed' refers to elements are of the same type and cannot be different from each other

pub fn run() {
    let mut arr = [1, 2, 3, 10, 100, 1000];

    println!("{:?}", arr);

    println!("The size of the array: {} bytes", mem::size_of_val(&arr));

    let slice = &arr[1..3];
    println!("The slice of the array is {:?}", slice);
    println!("and the size of the slice is {}", mem::size_of_val(slice));
}
