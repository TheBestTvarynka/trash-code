#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::multiple_unsafe_ops_per_block)]

use std::slice::from_raw_parts_mut;

use libc::{c_char, c_int, size_t};

pub const SIMPLELIB_SUCCESS: c_int = 0;
pub const SIMPLELIB_ERROR_NULL_POINTER: c_int = 1;
pub const SIMPLELIB_ERROR_INVALID_LENGTH: c_int = 2;

const HOSTNAME: &str = "thebesttvarynka";

/// Writes the current machine hostname into the provided buffer.
///
/// The resulting hostname length is written into the `length` parameter **without the NULL terminator char**.
///
/// # Safety
///
/// * The `name` pointer must not be null and point to the properly initialized and aligned memory.
/// * The `length` pointer must not be null and contain the `name` buffer size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_hostname(name: *mut c_char, length: *mut size_t) -> c_int {
    if name.is_null() || length.is_null() {
        return SIMPLELIB_ERROR_NULL_POINTER;
    }

    // SAFETY: `length` is not NULL due to prior check.
    let buf_length = unsafe { *length };
    let hostname_len = HOSTNAME.len();
    if hostname_len > buf_length {
        return SIMPLELIB_ERROR_INVALID_LENGTH;
    }

    // SAFETY:
    // - `name` is not NULL due to prior check.
    // - The caller must ensure that the memory is properly initialized and aligned.
    let buf = unsafe { from_raw_parts_mut(name as *mut u8, hostname_len) };
    buf.copy_from_slice(HOSTNAME.as_bytes());
    // SAFETY: `length` is not NULL due to prior check.
    unsafe {
        *length = hostname_len;
    }

    0
}
