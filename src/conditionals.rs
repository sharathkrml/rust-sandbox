// if-else
pub fn run() {
    let age = 18;
    let have_id = false;
    if age >= 21 && have_id {
        println!("age is ok!");
    } else if age < 21 && have_id {
        println!("age is less");
    } else {
        println!("get an ID");
    }
    // shorthand
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age {}", is_of_age)
}
