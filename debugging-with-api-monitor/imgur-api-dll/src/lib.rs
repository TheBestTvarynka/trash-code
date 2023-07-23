mod imgur;

pub use imgur::{FfiCommentData, FiiComment};

use std::ffi::CStr;

use imgur_api_client::ImgurApi;
use libc::{c_void, c_char, c_ulonglong};

#[no_mangle]
pub unsafe extern "C" fn ImgurInitClient(client_id: *const c_char, client_secret: *const c_char) -> *mut c_void {
    let client_id = CStr::from_ptr(client_id);
    let client_secret = CStr::from_ptr(client_secret);

    let context = ImgurApi::init(client_id.to_str().unwrap(), client_secret.to_str().unwrap());

    Box::into_raw(Box::new(context)) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ImgurGetComment(context: *mut c_void, comment_id: c_ulonglong, comment: *mut *mut FiiComment) -> u32 {
    let context: Box<ImgurApi> = Box::from_raw(context as *mut _ );

    if let Ok(c) = context.comment(comment_id) {
        *comment = Box::into_raw(Box::new(c.into()));

        0
    } else {
        1
    }
}
