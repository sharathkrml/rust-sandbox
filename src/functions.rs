// functions - used to store blocks of code for reuse

pub fn run() {
    greeting("Sharath");
    // bind function value to bar
    let get_sum = add(5, 5);
    println!("sum {}", get_sum);

    // closure
    let n3 = 20;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum {}", add_nums(3, 4));
}
fn greeting(name: &str) {
    println!("Hello {}", name);
}
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
