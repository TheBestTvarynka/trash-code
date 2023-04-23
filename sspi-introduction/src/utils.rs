use std::slice::from_raw_parts;

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
