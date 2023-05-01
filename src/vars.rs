// variables hold primtive data or references to data
// variables are immutable by default
// rust is a block scoped lang

pub fn run() {
    let name = "Sharath";
    // mutable variable
    let mut age = 22;
    println!("My name is {},I am {}", name, age);
    age = 23;
    println!("My name is {},I am {}", name, age);

    // defining const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assigning multiple variables
    let (my_name, my_age) = ("Sharath", 23);
    println!("My name is {},I am {}", my_name, my_age);
}
