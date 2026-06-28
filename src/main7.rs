use clap::Parser;

#[derive(Parser)]
#[command(name = "Transaction Decoder")]
#[command(version = "1.0")]
#[command(about = "Bitcoin Transaction Decoder", long_about = None)]
struct Cli {
    #[arg(required = true, help = "(string, required) Raw transaction hex")]
    transaction_hex: String,
}

fn main() {
    let cli = Cli::parse();
    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}

// use clap::Parser;
// #[derive(Parser)] //allows us to parse command line input and return argument matches
// #[command(name = "Tx Decoder")]
// #[command(version = "1.0")]
// #[command(about = "A simple Bitcoin Decoder", long_about = None)]
// struct Cli {
//     #[arg(required = true, help = "(string,required)Raw tx hex")]
//     transaction_hex: String,
// }
// //main.rs is entry point into our program and lib.rs has no entry point but can be called anywhere else to be used.. so we are goin to move our code to lib.rs and call it in the main to be used
// fn main() //-> Result<(), Box<dyn Error>> //empty tuple or an error will be handled
// {
//     // Example transaction hex string (truncated for demo)

//     // let transaction_hex = "01000000024d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc29";
//     let cli = Cli::parse();
//     //cargo run --hello -> give error , cargo run --full hex -> gives exact tx
//     //why do we use that transaction-decoder and also taking back to putting functionality in seperete files
//     match transaction_decoder::decode(cli.transaction_hex) {
//         //hyphens are converted to underscores in rust
//         //(transaction_hex.to_string()) {
//         //match statement which decodes our transaction
//         Ok(json) => println!("Decoded Transaction: {}", json),
//         Err(e) => eprintln!("Error decoding transaction: {}", e),
//     }
//     // // let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();
//     // // Print parsed values
//     // println!("Tx: {}", serde_json::to_string_pretty(&transaction)?);
//     // // println!("Inputs {}", json_inputs);
//     // Ok(()) //we return an empty tuple for the program to compile correctly
// }
// /*  a “compressed integer” format - compact size ->Instead of always using 8 bytes, Bitcoin saves space by using fewer bytes for small numbers.
// variable : length format
// CompactSize is Bitcoin’s way of encoding integers in a variable ‑ length format
// If the number ≤ 252 → store it in 1 byte.
// If the number ≤ 65,535 → prefix with 0xFD and store it in 2 bytes (u16).
// If the number ≤ 4,294,967,295 → prefix with 0xFE and store it in 4 bytes (u32).
//  structures exist in other protocols:

// VarInt in protobuf.

// LEB128 in WebAssembly.

// Variable‑length quantity (VLQ) in MIDI files.

// */
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::error::Error;

//     #[test]
//     fn test_read_compact_size() -> Result<(), Box<dyn Error>> {
//         // Case 1: single byte
//         let mut bytes = [1_u8].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 1_u64);

//         // Case 2: marker 253 (0xfd) followed by 2 bytes
//         let mut bytes = [253_u8, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64);

//         // Case 3: marker 254 (0xfe) followed by 4 bytes
//         let mut bytes = [254_u8, 0, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64.pow(3));

//         // Case 4: marker 255 (0xff) followed by 8 bytes
//         let mut bytes = [255_u8, 0, 0, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64.pow(7));

//         // Case 5: hex string "fd204e" → 0x4e20 = 20000
//         let hex = "fd204e";
//         let decoded = hex::decode(hex)?;
//         let mut bytes = decoded.as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         let expected_count = 20_000_u64;
//         assert_eq!(count, expected_count);

//         Ok(())
//     }
// }
