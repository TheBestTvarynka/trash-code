mod error;
mod imgur;

pub use error::{Error, ImgurResult};
pub use imgur::{Comment, CommentData};

use reqwest::blocking::Client;

const AUTH_HEADER: &str = "Authorization";

pub struct ImgurApi {
    client_id: String,
    _client_secret: String,
}

impl ImgurApi {
    pub fn init<I, S>(client_id: I, _client_secret: S) -> Self
    where
        String: From<I>,
        String: From<S>,
    {
        Self {
            client_id: client_id.into(),
            _client_secret: _client_secret.into(),
        }
    }

    pub fn comment(&self, comment_id: u64) -> ImgurResult<Comment> {
        let raw_comment = Client::new()
            .get(format!("https://api.imgur.com/3/comment/{}", comment_id))
            .header(AUTH_HEADER, format!("Client-ID {}", self.client_id))
            .send()?
            .bytes()?;

        let comment: Comment = serde_json::from_slice(&raw_comment)?;

        Ok(comment)
    }
}

#[cfg(test)]
mod tests {
    use crate::ImgurApi;

    #[test]
    fn comment() {
        let imgur = ImgurApi::init(env!("CLIENT_ID"), env!("CLIENT_SECRET"));
        print!("{:?}", imgur.comment(1911999579));
    }
}
