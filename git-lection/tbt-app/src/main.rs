use std::slice::from_raw_parts;

pub unsafe fn c_w_str_to_string(s: *const u16) -> String {
    let mut len = 0;

    while unsafe { *(s.add(len)) } != 0 {
        len += 1;
    }

    String::from_utf16_lossy(unsafe { from_raw_parts(s, len) })
}

fn main() {
    println!("Tbt was here.");
}
