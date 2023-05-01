use std::fmt::format;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // get fullname
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 100;
    println!("Color Red:{} Green:{} Blue:{}", c.red, c.green, c.blue);

    let mut p = Person::new("Sharath", "V");
    p.set_last_name("Krml");
    println!("Person full_name: {}", p.full_name());
}
