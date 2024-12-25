use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum Error {
    #[error("invalid name: {0}")]
    InvalidName(String),

    #[error("Unexpected JS exception")]
    UnexpectedException,
}
