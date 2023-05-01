// Primtive string = Immutable,fixed size
// String: growable,heap-allocated data structure
pub fn run() {
    let hello = "Hello"; // immutable
    let mut hello_1 = String::from("Hello");
    // get length
    let hello_len = hello.len();
    let hello_1_len = hello_1.len();
    // pushing char
    hello_1.push(' ');
    hello_1.push('w');
    // pusing str
    hello_1.push_str("Orld");
    // replace
    // hello_1 = String::from("value");
    // capactiy in bytes
    println!("Capacity: {}", hello_1.capacity());
    // contains substring
    println!("Contains 'wOrld' {}", hello_1.contains("wOrld"));
    // Replace
    println!("Replace:{}", hello_1.replace("Hello", "Hola!"));
    println!("{:?}", (hello, hello_1, hello_len, hello_1_len));
    // Loop through string by whitespace
    let new_word = String::from("h e l l o");
    for word in new_word.split(" ") {
        println!("{}", word);
    }
    for word in new_word.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion testing
    assert_eq!(2, s.len());
}
