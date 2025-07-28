// Generated bindings:
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod simplelib;

use libc::{c_char, c_int, size_t};

// Manual bindings:
unsafe extern "C" {
    unsafe fn get_hostname(name: *mut c_char, length: *mut size_t) -> c_int;
}

fn main() {
    let mut hostname_len = 256;
    let mut hostname = vec![0_u8; hostname_len];

    // let result = unsafe { get_hostname(hostname.as_mut_ptr() as *mut c_char, &mut hostname_len) };
    let result = unsafe { simplelib::get_hostname(hostname.as_mut_ptr() as *mut c_char, &mut hostname_len) };
    if result != 0 {
        panic!("Failed to get hostname: status code: {result}.");
    }

    let hostname =
        str::from_utf8(&hostname[0..hostname_len]).expect("hostname should be UTF-8 valid string");
    println!("Hostname: {hostname}.");
}
