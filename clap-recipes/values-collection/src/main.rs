mod cipher;

use cipher::Cipher;
use clap::Parser;

/// Server config structure
#[derive(Parser, Debug)]
struct Config {
    /// Allowed encryption algorithms list (separated by ':' or space).
    #[arg(
        long,
        value_name = "CIPHERS",
        default_values_t = vec![Cipher::Aes256],
        value_delimiter = ':',
        num_args = 1..,        // at least one allowed cipher type
    )]
    pub enc_algs: Vec<Cipher>,
}

fn main() {
    println!("{:?}", Config::parse());
}
