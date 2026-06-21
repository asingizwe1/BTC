//bringing clap to scope to help displaying on commandline
use clap::{Parser, Subcommand};
struct Cli {
    command: Option<Commands>,
}

enum Commands {
    Getblockhash { height: u64 },
}

fn main() {
    println!("Hello, world!");
}
