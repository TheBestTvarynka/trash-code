#[derive(Debug)]
pub struct Error(pub String);

pub type ImgurResult<T> = Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self(format!("{:?}", err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self(format!("{:?}", err))
    }
}
