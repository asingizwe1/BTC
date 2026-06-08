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
    match prefix[0] {
        0..=252 => prefix[0] as u64,
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
    let input_length = read_compact_size(&mut bytes_slice);
    printlm!("input length: {}", input legth);
}
