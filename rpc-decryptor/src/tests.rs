use crate::security_buffer::{SecBuffer, DATA, TOKEN};
use crate::sspi::{KerberosClient, KerberosServer};

#[test]
fn encrypt_decrypt() {
    let tbt = b"TheBestTvarynka";
    let session_key = [
        91, 11, 188, 227, 10, 91, 180, 246, 64, 129, 251, 200, 118, 82, 109, 65, 241, 177, 109, 32, 124, 39, 127, 171,
        222, 132, 199, 199, 126, 110, 3, 166,
    ];
    let kerberos_client = KerberosClient::new(session_key.to_vec());
    let kerberos_server = KerberosServer::new(session_key.to_vec());

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
