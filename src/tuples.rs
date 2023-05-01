// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Sharath", "V", 23);
    println!(
        "First name {} Last name {} Age {}",
        person.0, person.1, person.2
    )
}
