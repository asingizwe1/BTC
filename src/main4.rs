use std::fmt;

// Define a struct
struct Person {
    name: String,
    age: u8,
}

// Implement the Display trait for Person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: String::from("Louis"),
        age: 21,
    };

    // Thanks to Display, we can use {}
    println!("{}", p);
}
