mod cipher;

use cipher::Cipher;
use clap::Parser;

/// Server config structure
#[derive(Parser, Debug)]
struct Config {
    /// Allowed encryption algorithms list
    #[arg(long, value_name = "ENCRYPTION ALGORITHMS")]
    pub enc_algs: Vec<Cipher>,
}

fn main() {
    println!("{:?}", Config::parse());
}
