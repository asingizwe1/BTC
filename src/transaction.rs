#[derive(Debug, Serialize)] //as long as all types that belong to it can be serialized
struct input {
    txid: String, // [u8; 32],
    output_index: u32,
    script_sig: String, //Vec<u8>,
    sequence: u32,
}

// impl Amount {
//     // pub fn to_btc(&self) -> f64 {
//     //     //self.0 ->calls first element which is f64
//     //     self.0 as f64 / 100_000_000.0
//     // }we  no longer need implementation for to_btc coz we are going to implement it for a trait to be used in the as_btc function
// }

#[derive(Debug, Serialize)]
struct Transaction {
    Version: u32, // [u8; 32],
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

//OUTPUT
#[derive(Debug, Serialize)]
struct Output {
    #[serde(serialize_with = "as_btc")]
    amount: Amount, //f64, //we want to keep this as amount type not as f64 -> we serialise it
    script_pubkey: String,
}
//our type will get cast into this method as variable t
//we are storing amount as amount type but displaying it as an f64
fn as_btc<S: Serializer, T: BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc(); //rust doesnt know about the types to be passed in this method
    s.serialize_f64(btc)
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
