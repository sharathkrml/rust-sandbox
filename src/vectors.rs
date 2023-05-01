// Vectors  - resizable arrays

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", (numbers));
    // Get value by index
    println!("{}", numbers[4]);
    // re-assign
    numbers[2] = 10;
    println!("{:?}", (numbers));
    // get Vec len
    println!("{}", numbers.len());
    // Vec are stack allocated
    println!("Vec occupies {} bytes", mem::size_of_val(&numbers));
    let numbers_1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Vec occupies {} bytes", mem::size_of_val(&numbers_1));
    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice {:?}", (slice));
    // add to vec
    numbers.push(10);
    numbers.push(11);
    println!("{:?}", (numbers));
    // pop last
    numbers.pop();
    println!("{:?}", (numbers));

    // Loop through
    for x in numbers.iter() {
        println!("elem: {}", x)
    }
    println!("before: {:?}", (numbers));
    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("after: {:?}", (numbers));
}
