struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom pointer with data: {}", self.data);
    } // takes a mutable reference as an argument
}
//owning ensures that there can be one owner
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"), //we create instances of our struct and bind them to c and d
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}
