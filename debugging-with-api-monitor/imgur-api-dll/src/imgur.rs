use std::ffi::CString;

use imgur_api_client::{Comment, CommentData};
use libc::c_char;

pub type FfiStr = *const c_char;

#[repr(C)]
pub struct FfiCommentData {
    id: u64,
    image_id: FfiStr,
    comment: FfiStr,
    author: FfiStr,
    author_id: u64,
    on_album: bool,
    album_cover: FfiStr,
    ups: u32,
    downs: u32,
    points: u32,
    datetime: u32,
    parent_id: u64,
    deleted: bool,
    is_voted: bool,
    vote: bool,
    platform: FfiStr,
    has_admin_badge: bool,
    children: *const u64,
    children_len: u32,
}

#[repr(C)]
pub struct FiiComment {
    pub data: FfiCommentData,
    pub status: u32,
    pub success: bool,
}

impl From<Comment> for FiiComment {
    fn from(comment: Comment) -> Self {
        let Comment { data, success, status } = comment;
        let CommentData { id, image_id, comment, author, author_id, on_album, album_cover, ups, downs, points, datetime, parent_id, deleted, vote, platform, has_admin_badge, children } = data;

        let image_id = unsafe { CString::from_vec_unchecked(image_id.into_bytes()).into_raw() };
        let comment = unsafe { CString::from_vec_unchecked(comment.into_bytes()).into_raw() };
        let author = unsafe { CString::from_vec_unchecked(author.into_bytes()).into_raw() };
        let album_cover = unsafe { CString::from_vec_unchecked(album_cover.into_bytes()).into_raw() };
        let platform = unsafe { CString::from_vec_unchecked(platform.into_bytes()).into_raw() };

        let (is_voted, vote) = vote.map(|v| (true, v)).unwrap_or_default();

        let children_len = children.len().try_into().unwrap();
        let children = Box::into_raw(children.into_boxed_slice()) as *const u64;

        Self {
            data: FfiCommentData {
                id,
                image_id,
                comment,
                author,
                author_id,
                on_album,
                album_cover,
                ups: ups.try_into().unwrap(),
                downs: downs.try_into().unwrap(),
                points: points.try_into().unwrap(),
                datetime: datetime.try_into().unwrap(),
                parent_id,
                deleted,
                is_voted,
                vote,
                platform,
                has_admin_badge,
                children,
                children_len,
            },
            status: status.try_into().unwrap(),
            success: success,
        }
    }
}
