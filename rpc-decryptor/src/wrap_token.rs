//! This module contains the implementation of the Kerberps Wrap Token.
//!
//! Specification: [RFC 4121](https://www.rfc-editor.org/rfc/rfc4121.html).

use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

/// [Wrap Tokens](https://www.rfc-editor.org/rfc/rfc4121.html#section-4.2.6.2).
///
/// Use of the GSS_Wrap() call yields a token (referred as the Wrap token
/// in this document), which consists of a descriptive header, followed
/// by a body portion that contains either the input user data in
/// plaintext concatenated with the checksum, or the input user data
/// encrypted.
#[derive(Debug)]
pub struct WrapTokenHeader {
    /// 2        Flags     Attributes field, as described in [section 4.2.2](https://www.rfc-editor.org/rfc/rfc4121.html#section-4.2.2).
    pub flags: u8,
    /// 4..5     EC        Contains the "extra count" field, in big-
    ///                    endian order as described in [section 4.2.3](https://www.rfc-editor.org/rfc/rfc4121.html#section-4.2.3).
    pub ec: u16,
    /// 6..7     RRC       Contains the "right rotation count" in big-
    ///                     endian order, as described in [section 4.2.5](https://www.rfc-editor.org/rfc/rfc4121.html#section-4.2.5).
    pub rrc: u16,
    /// 8..15    SND_SEQ   Sequence number field in clear text,
    ///                    expressed in big-endian order.
    pub send_seq: u64,
}

impl WrapTokenHeader {
    /// Encodes the Wrap Token header into a byte array.
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

    /// Decodes the Wrap Token header from a provided reader.
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
