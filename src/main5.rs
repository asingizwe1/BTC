#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p1 = Person {
        name: String::from("Louis"),
        age: 21,
    };

    // Debug lets us print with {:?}
    println!("Debug: {:?}", p1);

    // Clone makes a copy
    let p2 = p1.clone();

    // PartialEq + Eq let us compare
    if p1 == p2 {
        println!("They are equal!");
    } else {
        println!("They are different!");
    }
}
