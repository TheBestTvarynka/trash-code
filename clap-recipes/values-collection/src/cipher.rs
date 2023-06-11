use std::fmt::{self, Display, Formatter};

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Error,
};

const AES128: &str = "aes128";
const AES256: &str = "aes256";
const DES3: &str = "des3";

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Cipher {
    Aes128,
    Aes256,
    Des3,
}

impl AsRef<str> for Cipher {
    fn as_ref(&self) -> &str {
        match self {
            Cipher::Aes128 => AES128,
            Cipher::Aes256 => AES256,
            Cipher::Des3 => DES3,
        }
    }
}

impl Display for Cipher {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl TryFrom<&str> for Cipher {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            AES128 => Ok(Self::Aes128),
            AES256 => Ok(Self::Aes256),
            DES3 => Ok(Self::Des3),
            _ => {
                let mut error = Error::new(ErrorKind::InvalidValue);
                error.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String("enc-algs".into()),
                );
                error.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(format!("Invalid algorithm name: {}", value)),
                );
                Err(error)
            }
        }
    }
}
