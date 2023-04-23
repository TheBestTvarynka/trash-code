use std::{ptr::null_mut, slice::from_raw_parts};

use winapi::um::sspi::{SecBufferDesc, SecHandle};

pub unsafe fn c_wide_string_to_rs_string(s: *const u16) -> String {
    let mut len = 0;

    while *(s.add(len)) != 0 {
        len += 1;
    }

    String::from_utf16_lossy(from_raw_parts(s, len))
}

pub fn str_to_win_wstring(value: &str) -> Vec<u16> {
    value
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>()
}

pub fn vec_into_raw_ptr<T>(v: Vec<T>) -> *mut T {
    Box::into_raw(v.into_boxed_slice()) as *mut T
}

pub fn unwrap_sec_handle(sec_handle: &mut SecHandle) -> *mut SecHandle {
    if sec_handle.dwLower == 0 && sec_handle.dwUpper == 0 {
        null_mut()
    } else {
        sec_handle
    }
}

pub unsafe fn log_sec_buffer_desc(name: &str, sec_buffer_desk: &SecBufferDesc) {
    println!("{}", name);
    println!("version: {}", sec_buffer_desk.ulVersion);
    println!("buffers amount: {}", sec_buffer_desk.cBuffers);
    println!("Buffers:");
    for i in 0..sec_buffer_desk.cBuffers {
        let sec_buffer = sec_buffer_desk.pBuffers.add(i as usize);
        println!(
            "{} {:?}",
            (*sec_buffer).BufferType,
            from_raw_parts(
                (*sec_buffer).pvBuffer as *const u8,
                (*sec_buffer).cbBuffer as usize
            )
        );
    }
}
