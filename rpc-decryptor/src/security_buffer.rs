pub const DATA: u32 = 1;
pub const TOKEN: u32 = 2;

pub const SECBUFFER_ATTRMASK: u32 = 0xF0000000;
pub const READONLY_FLAG: u32 = 0x80000000;
pub const READONLY_WITH_CHECKSUM_FLAG: u32 = 0x10000000;

#[derive(Debug)]
pub struct SecBuffer<'data> {
    pub buffer_type: u32,
    pub data: &'data mut [u8],
}

impl<'data> SecBuffer<'data> {
    pub fn new(buffer_type: u32, data: &'data mut [u8]) -> Self {
        Self { buffer_type, data }
    }
}
