use picky_krb::crypto::aes::Aes256CtsHmacSha196;
use picky_krb::crypto::{Cipher, EncryptWithoutChecksum};
use rand::rngs::OsRng;
use rand::TryRngCore;

use crate::security_buffer::{SecBuffer, DATA, READONLY_WITH_CHECKSUM_FLAG, TOKEN};
use crate::wrap_token::WrapTokenHeader;

const EC: u16 = 16;
const RRC: u16 = 28;

const ENCRYPTION_KEY_USAGE: i32 = 24;
const DECRYPTION_KEY_USAGE: i32 = 22;
const CB_SECURITY_TRAILER: usize = 76;

pub struct KerberosClient {
    pub key: Vec<u8>,
}

impl KerberosClient {
    pub fn encrypt_message(&self, message: &mut [SecBuffer<'_>]) {
        let mut wrap_token_header = WrapTokenHeader {
            flags: 0x06,
            ec: EC,
            rrc: 0,
            send_seq: OsRng.try_next_u64().unwrap(),
        };
        let encoded_wrap_token_header = wrap_token_header.encoded();
        let filler = vec![0; usize::from(EC)];

        // Enc buffer
        let mut data_to_encrypt = message.iter().fold(Vec::new(), |mut acc, sec_buffer| {
            if sec_buffer.buffer_type == DATA {
                acc.extend_from_slice(sec_buffer.data);
            }

            acc
        });
        // + Filler
        data_to_encrypt.extend_from_slice(&filler);
        // + Wrap token header
        data_to_encrypt.extend_from_slice(&encoded_wrap_token_header);

        let cipher = Aes256CtsHmacSha196::new();

        let EncryptWithoutChecksum {
            mut encrypted,
            confounder,
            ki: _,
        } = cipher
            .encrypt_no_checksum(&self.key, ENCRYPTION_KEY_USAGE, &data_to_encrypt)
            .unwrap();

        let data_to_sign = message.iter().fold(confounder, |mut acc, sec_buffer| {
            if sec_buffer.buffer_type == DATA | READONLY_WITH_CHECKSUM_FLAG {
                acc.extend_from_slice(sec_buffer.data);
            }

            acc
        });
        // + Filler
        data_to_encrypt.extend_from_slice(&filler);
        // + Wrap token header
        data_to_encrypt.extend_from_slice(&encoded_wrap_token_header);

        let checksum = cipher
            .encryption_checksum(&self.key, ENCRYPTION_KEY_USAGE, &data_to_sign)
            .unwrap();

        encrypted.extend_from_slice(&checksum);

        encrypted.rotate_right(usize::from(RRC + EC));

        wrap_token_header.rrc = RRC;

        // final Wrap Token
        let mut raw_wrap_token = wrap_token_header.encoded().to_vec();
        raw_wrap_token.extend_from_slice(&encrypted);

        let (token, data) = raw_wrap_token.split_at_mut(CB_SECURITY_TRAILER);

        let token_buffer = message
            .iter_mut()
            .find(|sec_buffer| sec_buffer.buffer_type == TOKEN)
            .expect("TOKEN buffer not found");
        token_buffer.data.copy_from_slice(token);

        let data_buffer = message
            .iter_mut()
            .find(|sec_buffer| sec_buffer.buffer_type == DATA)
            .expect("DATA buffer not found");
        data_buffer.data.copy_from_slice(data);
    }

    pub fn decrypt_message(&self, _message: &mut [SecBuffer<'_>]) {
        //
    }
}
