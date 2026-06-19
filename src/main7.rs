use hex;
// use serde::{Serialize, Serializer}; //bringing serde serialize into scope to help in formatting into json format
use sha2::{Digest, Sha256};
use std::io::Read; // Needed to use the .read() method on byte slices // External crate for decoding hex strings into bytes

//Each input tells the network: “I’m spending this specific output from a previous transaction.”
// TxID (32 bytes)

// The hash of the previous transaction that created the output you’re spending.

// Vout / Output Index (4 bytes, u32)

// Which output of that transaction you’re spending.

// Example: If a transaction had 3 outputs, vout = 0 means the first output, vout = 1 means the second, etc.

// ScriptSig (variable length)

// A script that provides unlocking data (usually your digital signature + public key).

// Proves you own the coins.

// Sequence (4 bytes)

// Used for advanced features like Replace‑By‑Fee or timelocks.

// Often set to 0xFFFFFFFF if unused.
use std::io::{Error, Read};
// use serde::{Serialize, Serializer}; we move this to the transaction file
use transaction::{Amount, Input, Output, Transaction, Txid}; //may fail coz structs are private
mod transaction;
// Reads a CompactSize integer (Bitcoin’s variable‑length integer format)
//the error comes from the Read trait
fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    //u64 {
    let mut compact_size = [0u8; 1]; // Read the first marker byte
    transaction_bytes.read(&mut compact_size)?; //.unwrap(); //using ? propagates the error instead of panicking when you use unwrap
    //read_compact_size returns a u64 because Bitcoin’s CompactSize can represent very large numbers (up to 2^64).
    match compact_size[0] {
        //we need to map the expressions returned in the Ok
        0..=252 => Ok(compact_size[0] as u64), // Direct value
        253 => {
            let mut buffer = [0u8; 2]; // Next 2 bytes → u16
            transaction_bytes.read(&mut buffer)?; //.unwrap();
            Ok(u16::from_le_bytes(buffer) as u64)
        }
        254 => {
            let mut buffer = [0u8; 4]; // Next 4 bytes → u32
            transaction_bytes.read(&mut buffer)?; //.unwrap();
            Ok(u32::from_le_bytes(buffer) as u64)
        }
        255 => {
            let mut buffer = [0u8; 8]; // Next 8 bytes → u64
            transaction_bytes.read(&mut buffer)?; //.unwrap();
            Ok(u64::from_le_bytes(buffer)) //interprete the bytes as little endian
        }
    }
}
//we want a data structure to return 32 bytes thats an array hence return type
// Placeholder for reading a transaction ID (txid) — usually 32 bytes
fn read_txid(transaction_bytes: &mut &[u8]) -> Txid {
    // [u8; 32]
    let mut buffer = [0; 32]; // Allocate 32 bytes
    transaction_bytes.read(&mut buffer)?; //.unwrap(); // Read 32 bytes
    //we look up tx ids in big endian format
    // buffer.reverse(); //this reverses the bytes in place - also dont need to reverse it coz its getting serialized
    Txid::from_bytes(
        //hex::encode - we dont need to hex encode it
        buffer,
    ) //to return array - to encode the buffer as hex strings
    //next 4 bytes give us output index, we are spending from
    // Return the raw txid bytes
}

//mut &[u8] -> any dynamically sized contiguous data
//since the script size isnt known
fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, Error> {
    //-> Vec<u8> {
    //You can’t directly use u64 for indexing or allocation because Rust enforces type safety.
    let script_size = read_compact_size(&mut bytes_slice)? //? because read_compact_size was changed to return a result 
    as usize;
    let mut buffer = vec![0_u8; script_size]; //the length parameter must be a usize, not u64.
    //usize is the type Rust uses for memory sizes and indexing.
    /*On a 32‑bit system → usize is 32 bits.

    This ensures that when you say “make a vector of length N,” the compiler knows how to handle it safely in memory. */
    transaction_bytes.read(&mut buffer).unwrap();
    //if type doesnt match rust coul try to dereference the object
    hex::encode(buffer) // buffer
}
//sha 256 will always produce 32 bytes so we know size at compile time
fn hash_raw_transaction(raw_transaction: &[u8]) -> Result<Txid, Error> {
    // First SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(raw_transaction);
    let hash1 = hasher.finalize();

    // Second SHA-256 hash (double hashing)
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();
    //we make sure we are hashing twice
    // Convert to fixed-size array [u8; 32]
    Ok(Txid::from_bytes(hash2.into())) //convert hash to our expected return type ->32 byte array
}

//our type will get cast into this method as variable t
//we are storing amount as amount type but displaying it as an f64
fn as_btc<S: Serializer, T: BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc(); //rust doesnt know about the types to be passed in this method
    Ok(s.serialize_f64(btc))
}
// Implement the Debug trait manually
// impl fmt::Debug for Input {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Input")
//             .field("txid", &self.txid)
//             .field("output_index", &self.output_index)
//             .field("script_sig", &self.script_sig)
//             .field("sequence", &self.sequence)
//             .finish()
//     }
// }
// Reads the 4‑byte version field from the transaction
fn read_32(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {
    //making read_version more generic
    //we read 4 bytes and retruned u32 representing the version also the index works the same way
    let mut buffer = [0; 4]; // Allocate 4 bytes
    transaction_bytes.read(&mut buffer)?; // Read 4 bytes into buffer
    Ok(u32::from_le_bytes(buffer)) // Interpret them as little‑endian u32
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
    //making read_version more generic
    //we read 4 bytes and retruned u32 representing the version also the index works the same way
    let mut buffer = [0; 8]; // Allocate 8 bytes
    transaction_bytes.read(&mut buffer)?; // Read 8 bytes into buffer
    Ok(Amount::from_sat(u64::from_le_bytes(buffer))) // Interpret them as little‑endian u64
}

fn main() {
    // Example transaction hex string (truncated for demo)
    let transaction_hex = "01000000024d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc29";

    // Decode hex string into raw bytes
    let transaction_bytes = hex::decode(transaction_hex)?;

    // Turn Vec<u8> into a slice so we can read sequentially
    let mut bytes_slice = transaction_bytes.as_slice();

    // Read the version (first 4 bytes)
    let version = read_u32(&mut bytes_slice);

    // Read the input count (CompactSize format)
    let input_count = read_compact_size(&mut bytes_slice);
    let mut inputs = vec![];
    // Loop through each input and read its txid
    for _ in 0..input_count {
        //BUT ITS BETTER TO PUT THIS IN A SCTRUCT
        //INPUT COMPONENTS
        //for the output index we call u32
        let output_index = read_u32(&mut bytes_slice);
        let txid = read_txid(&mut bytes_slice);
        let script_sig = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);
        //we shall push the inputs isnto the inputs vec
        inputs.push(Input {
            txid: txid, //you can just remove key -value and just replace with values only
            output_index: output_index,
            script_sig: script_sig,
            sequence: sequence,
        });
    }
    //after input section we shall collect our outputs
    let output_count = read_compact_size(&mut bytes_slice);
    let mut outputs = vec![];
    // Loop through each input and read its txid
    for _ in 0..output_count {
        //BUT ITS BETTER TO PUT THIS IN A SCTRUCT
        //INPUT COMPONENTS
        //for the output index we call u32
        let amount = read_amount(&mut bytes_slice); //.to_btc; //since it was in satoshi's
        let script_pubkey = read_script(&mut bytes_slice);

        //we shall push the inputs isnto the inputs vec
        outputs.push(Input {
            amount, //you can just remove key -value and just replace with values only
            script_pubkey,
        });
    }

    // Read lock_time from the byte slice
    let lock_time = read_u32(&mut bytes_slice);

    // Compute the transaction ID by double SHA-256 hashing
    let transaction_id = hash_raw_transaction(&transaction_bytes);

    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
    };

    #[derive(Debug)]
    pub struct Txid([u8; 32]);

    impl Txid {
        pub fn from_bytes(bytes: [u8; 32]) -> Txid {
            Txid(bytes)
        }
    }

    impl Serialize for Txid {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut bytes = self.0.clone();
            bytes.reverse(); //we first clone so that we are just modifying a copy in memory not the actual data
            s.serialize_str(&hex::encode(bytes))
        }
    }

    // let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();
    // Print parsed values
    println!("Tx: {}", serde_json::to_string_pretty(&transaction)?);
    // println!("Inputs {}", json_inputs);
}
/*  a “compressed integer” format - compact size ->Instead of always using 8 bytes, Bitcoin saves space by using fewer bytes for small numbers.
variable : length format
CompactSize is Bitcoin’s way of encoding integers in a variable ‑ length format
If the number ≤ 252 → store it in 1 byte.
If the number ≤ 65,535 → prefix with 0xFD and store it in 2 bytes (u16).
If the number ≤ 4,294,967,295 → prefix with 0xFE and store it in 4 bytes (u32).
 structures exist in other protocols:

VarInt in protobuf.

LEB128 in WebAssembly.

Variable‑length quantity (VLQ) in MIDI files.

*/
