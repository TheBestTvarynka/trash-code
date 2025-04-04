use crate::security_buffer::{SecBuffer, DATA, READONLY_WITH_CHECKSUM_FLAG, TOKEN};
use crate::sspi::{KerberosClient, KerberosServer};

#[test]
fn encrypt_decrypt() {
    let tbt = b"TheBestTvarynka";
    let session_key = [
        91, 11, 188, 227, 10, 91, 180, 246, 64, 129, 251, 200, 118, 82, 109, 65, 241, 177, 109, 32, 124, 39, 127, 171,
        222, 132, 199, 199, 126, 110, 3, 166,
    ];
    let kerberos_client = KerberosClient::new(session_key.to_vec(), 681238048);
    let kerberos_server = KerberosServer::new(session_key.to_vec(), 681238048);

    let mut data_buf = tbt.to_vec();
    let mut token_buf = vec![0; KerberosClient::TOKEN_LEN];
    let mut message = vec![
        SecBuffer::new(DATA, &mut data_buf),
        SecBuffer::new(TOKEN, &mut token_buf),
    ];

    kerberos_client.encrypt_message(&mut message);
    kerberos_server.decrypt_message(&mut message);

    assert_eq!(message[0].data, tbt);
}

#[test]
fn decrypt_rpc_request() {
    let session_key = vec![
        19, 28, 59, 181, 9, 202, 41, 22, 25, 122, 144, 217, 9, 87, 170, 209, 72, 223, 145, 41, 12, 252, 9, 229, 45,
        218, 206, 161, 199, 216, 243, 53,
    ];
    let kerberos_server = KerberosServer::new(session_key, 41895117);

    let mut header = vec![
        5, 0, 0, 3, 16, 0, 0, 0, 60, 1, 76, 0, 1, 0, 0, 0, 208, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut enc_data = vec![
        12, 0, 202, 158, 245, 73, 200, 196, 158, 187, 0, 135, 112, 187, 78, 253, 225, 60, 114, 70, 163, 77, 66, 75,
        203, 192, 9, 184, 3, 167, 100, 222, 182, 217, 253, 203, 107, 132, 172, 128, 70, 81, 210, 183, 199, 117, 14, 7,
        215, 161, 70, 44, 155, 123, 215, 95, 187, 68, 64, 145, 45, 106, 53, 92, 218, 167, 32, 113, 41, 179, 199, 93,
        184, 19, 95, 44, 40, 91, 135, 207, 72, 105, 99, 217, 231, 6, 167, 98, 66, 15, 121, 183, 134, 103, 196, 161, 76,
        117, 188, 160, 178, 41, 63, 253, 98, 127, 97, 67, 66, 175, 212, 255, 232, 212, 49, 154, 253, 229, 120, 25, 195,
        157, 4, 167, 14, 203, 124, 241, 195, 102, 98, 45, 123, 75, 181, 38, 240, 116, 19, 47, 14, 200, 210, 153, 60,
        194, 102, 179, 76, 65, 224, 161, 248, 169, 152, 208, 222, 177, 167, 43, 19, 130, 168, 105, 24, 173, 44, 97, 84,
        13, 1, 191, 57, 214, 26, 41, 205, 106, 74, 58, 194, 114, 107, 110, 175, 16, 90, 183, 185, 21, 126, 100, 197,
        127, 205, 114, 220, 100, 254, 60, 27, 170, 191, 139, 219, 3, 136, 73,
    ];
    let mut sec_trailer_header = vec![16, 6, 8, 0, 0, 0, 0, 0];
    let mut sec_trailer_auth_value = vec![
        5, 4, 6, 255, 0, 16, 0, 28, 0, 0, 0, 0, 2, 127, 68, 205, 187, 173, 74, 166, 160, 153, 40, 89, 22, 35, 47, 177,
        219, 87, 47, 250, 226, 224, 221, 190, 242, 117, 26, 8, 246, 227, 66, 83, 51, 25, 251, 201, 147, 221, 150, 156,
        38, 106, 201, 152, 57, 242, 145, 76, 4, 83, 52, 239, 115, 245, 174, 74, 173, 95, 157, 251, 98, 108, 134, 71,
    ];
    let mut message = vec![
        SecBuffer::new(DATA | READONLY_WITH_CHECKSUM_FLAG, &mut header),
        SecBuffer::new(DATA, &mut enc_data),
        SecBuffer::new(DATA | READONLY_WITH_CHECKSUM_FLAG, &mut sec_trailer_header),
        SecBuffer::new(TOKEN, &mut sec_trailer_auth_value),
    ];

    kerberos_server.decrypt_message(&mut message);

    println!("Plaintext: {:?}", message[1]);
}
