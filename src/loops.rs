pub fn run() {
    let mut count = 0;
    // infinite loop
    loop {
        count += 1;
        println!("num :{}", count);
        if count == 20 {
            break;
        }
    }
    // while loop
    while count < 100 {
        println!("count {}", count);
        count += 1;
    }
    // For Range loop
    for x in 50..53 {
        println!("count {}", x);
    }
}
