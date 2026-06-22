//bringing clap to scope to help displaying on commandline
use clap::{Parser, Subcommand};

#[derive(Parser)] //allows us to parse command line input and return argument matches
#[command(name = "Bitcoin CLI")]
#[command(version = "1.0")]
#[command(about = "A simple Bitcoin RPC client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    //Returns hash of clock in best-block-chain at height prvoided
    Getblockhash {
        //we use arg attribute to add extra information about the argument
        #[arg(required = true, help = "(numeric,required) The height index")]
        height: u64,
    },
}

fn main() {
    let cli = Cli::parse(); //parses the command line arguments and returns a Cli struct
    //MATCH TO SEE WHICH SUBCOMMAND USER PROVIDED
    match &cli.command {
Some(Commands::Getblockhash{height})//main pattern is the getblockhash
=>println!(),
None=>{eprint!("Error: Too few parameters")}
    }
}
//cargo run --getblockhash 1  , with out the argument 1 we get an error as described in the match statement
