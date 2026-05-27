
#[allow(unused)]
fn read_version(transaction_hex:&str)->u32{
    let transaction_bytes=hex::decode(transaction_hex).expect("Invalid hex string");
    let version_bytes=<u8;4>::try_from(&transaction_bytes[0..4]).unwrap();//for first 4 bytes

    
    //we unwrap because it is returning a result type, and we want to get the value out of the result type, if it is an error, we want to panic and print the error message.
u32::from_le_bytes(version_bytes)//     .try_into().expect("Failed to convert version bytes to u32"));
//    println!("version bytes: {:?}", version_bytes);//not all types implement display trait
   //:? says we must implement using debug trait//debug output solved by using {:?} instead of {} in println! macro
   // return 1;
    //you can as well write as 1 since the last statement after colon will be returned
//1   //idiomatic return
}
//pointer points and tells rust where to locate that heap data since pointer points to stack mem address
//enum recap
//Enum X{
//    A(String), //the value can also take up a type 
//    B,}  //let v =X::A("G's.to_string());")
//this is because any type in quotes isnt a String type at &str type
//we can use if let to print out the value of the enum variant, if let X::A(value) = v {
//if let X::A(value) = v {// if variable v can be distructured into type X::A(value)
//    println!("Value: {}", value);
//} //
//match v {
//    X::A(value) => println!("Value: {}", value),
//    X::B => println!("Variant B"), //you dont need brackets
//_ =>println!("none") ;}// you must cater for all missing cases
//For rsult Enum we must define types clearly
#[allow(unused)]
fn main() {
   let version = read_version("0200000001b1ed1c8e5b9a1c3f0e5b6a7c8d9e0f1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1u2v3w4x5y6z7a8b9c0d1e2f3g4h5i6j7k8l9m0n1o2p3q4r5s6t7u8v9w0x1y2z3a4b5c6d7e8f9g0h1i2j3k4l5m6n7o8p9q0r1s2t3u4v5w6x7y8z9a0b1c2d3e4f5g6h7i8j9k0l1m2n3o4p5q6r7s8t9u0v1w2x3y4z5a6b7");
println!("Version: {}", version);
   }
   
//we use a vec instead of array coz of stack allocation issues, we can use array if we know the size beforehand, but in this case we don't know the size of the transaction hex string, so we use a vec to store the bytes of the transaction.
//converting a hex string into vec
//we use the hex crate to convert the hex string into bytes, we can use the from_hex function to convert the hex string into a vec of bytes, and then we can use the vec of bytes to read the version of the transaction.
//Array [T;N] is a fixed-size array, where T is the type of the elements and N is the number of elements in the array. Vec<T> is a growable array, where T is the type of the elements. The main difference between the two is that an array has a fixed size, while a vec can grow or shrink in size as needed. In this case, we use a vec because we don't know the size of the transaction hex string beforehand, and we need to be able to store all the bytes of the transaction.
//TRYFROM trait copies and array from a slice, and it returns a Result type that indicates whether the conversion was successful or not. The try_into method is used to convert the slice of bytes into an array of bytes, and it will return an error if the conversion fails. In this case, we use try_into to convert the slice of bytes into an array of 4 bytes, which is the size of the version field in the transaction. If the conversion is successful, we can then use from_le_bytes to convert the array of bytes into a u32 value that represents the version of the transaction. and returns Result