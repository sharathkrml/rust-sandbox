pub fn run() {
    // printing
    println!("Hello,run!!");

    // formatting
    println!("Name: {} age: {}", "Brad", 23);

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes {2}",
        "Brad", "France", "Code"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "cricket"
    );

    // placeholder traits
    println!("Binary:{:b} Hex:{:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hi"));

    // basic math
    println!("10+10={}", 10 + 10);
}
