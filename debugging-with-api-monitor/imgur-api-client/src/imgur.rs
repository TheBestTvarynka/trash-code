use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentData {
    pub id: u64,
    pub image_id: String,
    pub comment: String,
    pub author: String,
    pub author_id: u64,
    pub on_album: bool,
    pub album_cover: String,
    pub ups: usize,
    pub downs: usize,
    pub points: usize,
    pub datetime: u64,
    pub parent_id: u64,
    pub deleted: bool,
    pub vote: Option<bool>,
    pub platform: String,
    pub has_admin_badge: bool,
    pub children: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub data: CommentData,
    pub success: bool,
    pub status: usize,
}
