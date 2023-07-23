use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentData {
    id: u64,
    image_id: String,
    comment: String,
    author: String,
    author_id: u64,
    on_album: bool,
    album_cover: String,
    ups: usize,
    downs: usize,
    points: usize,
    datetime: u64,
    parent_id: u64,
    deleted: bool,
    vote: Option<bool>,
    platform: String,
    has_admin_badge: bool,
    children: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub data: CommentData,
    pub success: bool,
    pub status: usize,
}
