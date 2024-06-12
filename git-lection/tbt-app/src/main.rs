use std::slice::from_raw_parts;

pub unsafe fn c_w_str_to_string(s: *const u16) -> String {
    let mut len = 0;

    while unsafe { *(s.add(len)) } != 0 {
        len += 1;
    }

    String::from_utf16_lossy(unsafe { from_raw_parts(s, len) })
}

fn prepate_data() -> *const u16 {
    Box::into_raw("Tbt\0"
        .encode_utf16()
        .collect::<Vec<_>>()
        .into_boxed_slice()) as *const _
}

fn main() {
    println!("{}", unsafe {
        c_w_str_to_string(prepate_data())
    });
}
