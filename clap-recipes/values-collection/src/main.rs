mod cipher;

use std::fmt;

use cipher::Cipher;
use clap::{builder::ValueParser, error::ErrorKind, Error, Parser};

#[derive(Debug, Clone)]
struct EncAlgorithms(pub Vec<Cipher>);

impl fmt::Display for EncAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut algs = self.0.iter();
        let result = algs.next().map(ToString::to_string).unwrap_or_default();

        let result = algs.fold(result, |mut result, alg| {
            result.push(':');
            result.push_str(alg.as_ref());
            result
        });

        f.write_str(&result)
    }
}

fn parse_encryption_algorithms(raw_enc_algorithms: &str) -> Result<EncAlgorithms, Error> {
    let mut parsed_enc_algs = Vec::new();

    for raw_enc_alg in raw_enc_algorithms.split(':').filter(|e| !e.is_empty()) {
        parsed_enc_algs.push(raw_enc_alg.try_into()?);
    }

    if parsed_enc_algs.is_empty() {
        return Err(Error::new(ErrorKind::InvalidValue));
    }

    Ok(EncAlgorithms(parsed_enc_algs))
}

/// Server config structure
#[derive(Parser, Debug)]
struct Config {
    /// Allowed encryption algorithms list (separated by ':').
    #[arg(
        long,
        value_name = "ENCRYPTION ALGORITHMS LIST",
        value_parser = ValueParser::new(parse_encryption_algorithms),
        default_value_t = EncAlgorithms(vec![Cipher::Aes256, Cipher::Aes128]),
    )]
    pub enc_algs: EncAlgorithms,
}

fn main() {
    println!("{:?}", Config::parse());
}
