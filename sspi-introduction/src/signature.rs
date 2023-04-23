use winapi::um::sspi::{MakeSignature, SecBuffer, SecBufferDesc, SecHandle};

use crate::utils::{log_sec_buffer_desc, vec_into_raw_ptr};

pub unsafe fn client_make_signature(message: &[u8], client_security_context: &mut SecHandle) {
    let sequence_number = 0;

    let mut message = SecBufferDesc {
        ulVersion: 0,
        cBuffers: 2,
        pBuffers: vec_into_raw_ptr(vec![
            SecBuffer {
                cbBuffer: 16,
                // Token
                BufferType: 2,
                pvBuffer: vec_into_raw_ptr(vec![0_u8; 16]) as *mut _,
            },
            SecBuffer {
                cbBuffer: message.len() as u32,
                // Data
                BufferType: 1,
                pvBuffer: vec_into_raw_ptr(message.to_vec()) as *mut _,
            },
        ]),
    };

    let status = MakeSignature(client_security_context, 0, &mut message, sequence_number);

    if status != 0 {
        panic!("Can not make signature. Error code: {:0x?}", status,);
    }

    log_sec_buffer_desc("Signature", &message);
}
