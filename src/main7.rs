use hex;
use std::io::Read; // Needed to use the .read() method on byte slices // External crate for decoding hex strings into bytes

// Reads the 4‑byte version field from the transaction
fn read_32(transaction_bytes: &mut &[u8]) -> u32 {
    //making read_version more generic
    //we read 4 bytes and retruned u32 representing the version also the index works the same way
    let mut buffer = [0; 4]; // Allocate 4 bytes
    transaction_bytes.read(&mut buffer).unwrap(); // Read 4 bytes into buffer
    u32::from_le_bytes(buffer) // Interpret them as little‑endian u32
}

// Reads a CompactSize integer (Bitcoin’s variable‑length integer format)
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0u8; 1]; // Read the first marker byte
    transaction_bytes.read(&mut compact_size).unwrap();
    //read_compact_size returns a u64 because Bitcoin’s CompactSize can represent very large numbers (up to 2^64).
    match compact_size[0] {
        0..=252 => compact_size[0] as u64, // Direct value
        253 => {
            let mut buffer = [0u8; 2]; // Next 2 bytes → u16
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        }
        254 => {
            let mut buffer = [0u8; 4]; // Next 4 bytes → u32
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        }
        255 => {
            let mut buffer = [0u8; 8]; // Next 8 bytes → u64
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer) //interprete the bytes as little endian
        }
    }
}
//we want a data structure to return 32 bytes thats an array hence return type
// Placeholder for reading a transaction ID (txid) — usually 32 bytes
fn read_txid(transaction_bytes: &mut &[u8]) -> [u8; 32] {
    let mut buffer = [0; 32]; // Allocate 32 bytes
    transaction_bytes.read(&mut buffer).unwrap(); // Read 32 bytes
    //we look up tx ids in big endian format
    buffer.reverse(); //this reverses the bytes in place
    buffer //to return array
    //next 4 bytes give us output index, we are spending from
    // Return the raw txid bytes
}

//mut &[u8] -> any dynamically sized contiguous data
//since the script size isnt known
fn read_script(transaction_bytes: &mut &[u8]) -> Vec<u8> {
    let script_size = read_compact_size(&mut bytes_slice) as usize;
    let mut buffer = vec![0_u8; script_size]; //the length parameter must be a usize, not u64.
    //usize is the type Rust uses for memory sizes and indexing.
    /*On a 32‑bit system → usize is 32 bits.

    This ensures that when you say “make a vector of length N,” the compiler knows how to handle it safely in memory. */
    transaction_bytes.read(&mut buffer).unwrap();
    //if type doesnt match rust coul try to dereference the object
    buffer
}

fn main() {
    // Example transaction hex string (truncated for demo)
    let transaction_hex = "01000000024d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc29";

    // Decode hex string into raw bytes
    let transaction_bytes = hex::decode(transaction_hex).unwrap();

    // Turn Vec<u8> into a slice so we can read sequentially
    let mut bytes_slice = transaction_bytes.as_slice();

    // Read the version (first 4 bytes)
    let version = read_u32(&mut bytes_slice);

    // Read the input count (CompactSize format)
    let input_count = read_compact_size(&mut bytes_slice);

    // Loop through each input and read its txid
    for _ in 0..input_count {
        //for the output index we call u32
        let output_index = read_u32(&mut bytes_slice);
        let txid = read_txid(&mut bytes_slice);
    }

    // Print parsed values
    println!("Version: {}", version);
    println!("Input count: {}", input_count);
}
