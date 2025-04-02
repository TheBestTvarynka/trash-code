use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

pub struct WrapTokenHeader {
    pub flags: u8,
    pub ec: u16,
    pub rrc: u16,
    pub send_seq: u64,
}

impl WrapTokenHeader {
    pub fn encoded(&self) -> [u8; 16] {
        let mut header_data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        header_data[0..2].copy_from_slice(&[0x05, 0x04]);
        header_data[2] = self.flags;
        header_data[3] = 0xff;
        header_data[4..6].copy_from_slice(&self.ec.to_be_bytes());
        header_data[6..8].copy_from_slice(&self.rrc.to_be_bytes());
        header_data[8..].copy_from_slice(&self.send_seq.to_be_bytes());

        header_data
    }

    pub fn from_bytes(mut src: impl Read) -> Self {
        if src.read_u16::<BigEndian>().unwrap() != 0x0504 {
            panic!("Invalid Wrap Token ID");
        }

        let flags = src.read_u8().unwrap();

        let filler = src.read_u8().unwrap();
        if filler != 0xff {
            panic!("Invalid filler");
        }

        let ec = src.read_u16::<BigEndian>().unwrap();
        let rrc = src.read_u16::<BigEndian>().unwrap();
        let send_seq = src.read_u64::<BigEndian>().unwrap();

        Self {
            flags,
            ec,
            rrc,
            send_seq,
        }
    }
}
