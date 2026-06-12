use std::io::Read;
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64
//since we cant know how large it will be
{
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    // if (0..253).contains(&compact_size[0]) {
    //     compact_size[0] as u64
    // }
    // //u8 can only have a value btn 0 and 255
    // //for the case of read following bytes and interprete it as u16
    // else if compact_size[0] == 253 {
    //     let mut buffer = [0; 2];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u16::from_le_bytes(buffer) as u64
    // } else if compact_size[0] == 254 {
    //     let mut buffer = [0; 4];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u16::from_le_bytes(buffer) as u64
    // } else {
    //     let mut buffer = [0; 8];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u16::from_le_bytes(buffer) as u64
    // }
    match compact_size[0] {
        //prefix is just a variable name just like saying match x
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0u8; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        }
        254 => {
            let mut buffer = [0u8; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        }
        255 => {
            let mut buffer = [0u8; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }
    }
}
fn main() {
    let transaction_bytes = vec![1u8];
    let mut bytes_slice = transaction_bytes.as_slice();
    let input_length = read_compact_size(&mut bytes_slice);

    println!("input length: {}", input_length);
}
//version tells Bitcoin software how to interpret the rest of the data
//hr proper way to add unit tests is to add the cfg attribute

//configuration check that tests rust to check when running in test mode
#[cfg(test)]
mod test {
    //place out test code in a sepereate module
    //test annotation - we place it above every function we want to run as a test
    #[test]
    fn test_read_compact_size() {
        //since the other functions are out of this scope we can use use::super;
        use super::read_compact_size;
        // Create a byte slice with a single value
        let mut bytes = [1_u8].as_slice();

        // Call the function under test
        let count = read_compact_size(&mut bytes);
        //function reads the first byte (1) and interprets it according to Bitcoin’s CompactSize rules:
        // Assert that the result matches the expected value
        assert_eq!(count, 1_u64);

        // Case 2: Marker 253 → next 2 bytes are a u16
        let mut bytes = [253_u8, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64);

        // Case 3: Marker 254 → next 4 bytes are a u32
        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(3));

        // Case 4: Marker 255 → next 8 bytes are a u64
        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(7));
    }
}
