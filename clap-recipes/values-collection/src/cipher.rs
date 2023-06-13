use std::fmt::{self, Display, Formatter};

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
