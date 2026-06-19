#[derive(Debug, Serialize)] //as long as all types that belong to it can be serialized
use serde::{Serialize, Serializer};
pub struct input {
    pub txid: String, // [u8; 32],
    pub output_index: u32,
    pub script_sig: String, //Vec<u8>,
    pub sequence: u32,
}

// impl Amount {
//     // pub fn to_btc(&self) -> f64 {
//     //     //self.0 ->calls first element which is f64
//     //     self.0 as f64 / 100_000_000.0
//     // }we  no longer need implementation for to_btc coz we are going to implement it for a trait to be used in the as_btc function
// }

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub Version: u32, // [u8; 32],
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: u32,
}

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

#[derive(Debug)]
pub struct Amount(pub u64);

impl Amount {
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi) //this is a type associated function not an instance method 
    }
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

//OUTPUT
#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount, //f64, //we want to keep this as amount type not as f64 -> we serialise it
    pub script_pubkey: String,
}
