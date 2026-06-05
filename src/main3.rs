fn main() {
    let arr = [0, 1, 2, 3];
    let len = calculate_len(arr);
    //for an array this wont fail even when arr had been owned by len
    //this is because an array implements the copy trait meaning it is stack allocated item so
    //basically all data is copied when passed into a function
    //applies to all stack allocated types
    println!("The len of {:?} is {}.", arr, len);
}

fn calculate_len(mut arr: [u8; 4]) -> usize {
    arr.len()
}
//at any point you can have one writer or mulitple readers
//you can have a SINGLE mutable reference, you cant read/ modify a vector at the same time
//you cant have a reference and a mutable reference of the same object at the same time
//   your wallet does not have a single "account balance". Instead,
//   your balance is a collection of discrete chunks of Bitcoin that were sent to you. */
//It serves as an exact "map" to locate the source funds you are attempting to spend
//in btc every transaction input points to a previous output - pointer has 2 pieces of info which are txid(String) and vout(output index)(u32)-[because you can have multiple transactions] - output index in that transaction *Because Bitcoin uses the UTXO (Unspent Transaction Output) model,
