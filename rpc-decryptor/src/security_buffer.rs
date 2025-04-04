//! Security Buffer implementation.
//!
//! MSDN:
//! * `SecBuffer` structure (`sspi.h`): https://learn.microsoft.com/en-us/windows/win32/api/sspi/ns-sspi-secbuffer

/// SECBUFFER_DATA - The buffer contains common data. The security package can read and write this data, for example,
/// to encrypt some or all of it.
pub const DATA: u32 = 1;
/// SECBUFFER_TOKEN - The buffer contains the security token portion of the message.
/// This is read-only for input parameters or read/write for output parameters.
pub const TOKEN: u32 = 2;

/// SECBUFFER_READONLY_WITH_CHECKSUM - The buffer is read-only with a checksum.
pub const READONLY_WITH_CHECKSUM_FLAG: u32 = 0x10000000;

/// The SecBuffer structure describes a buffer allocated by a transport application to pass to a security package.
#[derive(Debug)]
pub struct SecBuffer<'data> {
    /// Bit flags that indicate the type of buffer.
    pub buffer_type: u32,
    /// The buffer data.
    pub data: &'data mut [u8],
}

impl<'data> SecBuffer<'data> {
    /// Creates a new [SecBuffer] instance.
    pub fn new(buffer_type: u32, data: &'data mut [u8]) -> Self {
        Self { buffer_type, data }
    }
}
