// Arrays  - Fixed list where elements are the same data types

use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", (numbers));
    // Get value by index
    println!("{}", numbers[4]);
    // re-assign
    numbers[2] = 10;
    println!("{:?}", (numbers));
    // get array len
    println!("{}", numbers.len());
    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    let numbers_1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Array occupies {} bytes", mem::size_of_val(&numbers_1));
    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice {:?}", (slice));
}
