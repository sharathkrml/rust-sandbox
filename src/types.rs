// Primitive
// int:u8,i8,u16,i16, ... u128,i128(no of bits they take in mem)
// Float: f32,f64
// Boolean
// Characters
// Tuples
// Arrays

// Rust is a statically typed lang
// It must know the types of all variables at compile time
// Compiler can usually infer what type we want to use based on the value & how we use it
pub fn run() {
    let x = 1;
    let y = 3.2;
    // adding explicit type
    let z: i64 = 1256378;
    // max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);
    // Boolean
    let is_active = true;
    // Get bool from expression
    let is_greater = 10 > 15;
    // char: unicode char (should be single quote) (emojis ar unicode)
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
