use std::{ptr::null_mut, slice::from_raw_parts};

use num_traits::cast::ToPrimitive;
use sspi::{
    AuthIdentityBuffers, DataRepresentation, Ntlm, SecurityBuffer, SecurityBufferType,
    SecurityStatus, ServerRequestFlags, Sspi,
};
use winapi::um::sspi::{
    CredHandle, InitializeSecurityContextW, QueryContextAttributesW, SecBuffer, SecBufferDesc,
    SecHandle, SecPkgContext_SessionKey, TimeStamp, SECPKG_ATTR_SESSION_KEY,
};

use crate::utils::{
    log_sec_buffer_desc, str_to_win_wstring, unwrap_sec_handle, vec_into_raw_ptr,
    win_sec_buff_desc_to_sspi_sec_buffers,
};

const OK: i32 = 0;
const CONTINUE_NEEDED: i32 = 0x0009_0312;
const COMPLETE_NEEDED: i32 = 0x0009_0313;
const COMPLETE_AND_CONTINUE: i32 = 0x0009_0314;

/// Performs client-server authentication.
/// Returns security handle to the established client security context.
pub unsafe fn authenticate(
    mut client_credentials_handle: CredHandle,

    mut server_credentials_handle: Option<AuthIdentityBuffers>,
    server: &mut Ntlm,
) -> SecHandle {
    let mut client_security_context = SecHandle::default();

    // we don't have any input buffers on the first iteration
    let mut client_input_buffers = SecBufferDesc {
        ulVersion: 0,
        cBuffers: 0,
        pBuffers: null_mut(),
    };

    let mut target_name = str_to_win_wstring("TERMSRV\\testuser");

    loop {
        println!("==================================");

        let mut new_client_security_context = SecHandle::default();
        let mut expiry = TimeStamp::default();
        let mut context_attributes = 0;
        let mut client_output_buffers = SecBufferDesc {
            ulVersion: 0,
            cBuffers: 1,
            pBuffers: vec_into_raw_ptr(vec![SecBuffer {
                cbBuffer: 2888,
                BufferType: 2,
                pvBuffer: vec_into_raw_ptr(vec![0_u8; 2888]) as *mut _,
            }]),
        };

        let client_status = InitializeSecurityContextW(
            &mut client_credentials_handle,
            unwrap_sec_handle(&mut client_security_context),
            target_name.as_mut_ptr(),
            // MUTUAL_AUTH and ALLOCATE_MEMORY
            0x2 | 0x100,
            0,
            // Native data representation:
            0x10,
            &mut client_input_buffers,
            0,
            &mut new_client_security_context,
            &mut client_output_buffers,
            &mut context_attributes as *mut i32 as *mut _,
            &mut expiry as *mut _,
        );

        log_sec_buffer_desc("Client output buffers", &client_output_buffers);
        println!("Client status code: {:0x?}", client_status);

        if client_status != OK
            && client_status != CONTINUE_NEEDED
            && client_status != COMPLETE_NEEDED
            && client_status != COMPLETE_AND_CONTINUE
        {
            panic!(
                "Can not initialize security context. Status code: {:0x?}",
                client_status
            );
        }

        if new_client_security_context.dwLower != 0 || new_client_security_context.dwUpper != 0 {
            println!("Update client security context handle.");
            client_security_context = new_client_security_context;
        }

        if client_output_buffers.pBuffers.is_null() {
            panic!("Something went wrong. Client output buffers is NULL.");
        }

        // Now we send output buffers to the NTLM server
        //
        // +--------+                                                           +--------+
        // | client | =[client_output_buffers]=> .... =[server_input_buffers]=> | server |
        // +--------+                                                           +--------+
        //
        // Convert Windows security buffers to the sspi-rs security buffers.
        let mut server_input_buffers =
            win_sec_buff_desc_to_sspi_sec_buffers(&client_output_buffers);
        let mut server_output_buffers = vec![SecurityBuffer::new(
            Vec::with_capacity(1024),
            SecurityBufferType::Token,
        )];

        let server_status = server
            .accept_security_context()
            .with_credentials_handle(&mut server_credentials_handle)
            .with_context_requirements(ServerRequestFlags::empty())
            .with_target_data_representation(DataRepresentation::Native)
            .with_input(&mut server_input_buffers)
            .with_output(&mut server_output_buffers)
            .execute()
            .unwrap();
        println!("server accept security context result: {server_status:?}");

        println!("Server output buffers: {:?}", server_output_buffers);

        // Now we send output buffers to the NTLM client
        //
        // +--------+                                                           +--------+
        // | client | <=[client_input_buffers]= .... <=[server_output_buffers]= | server |
        // +--------+                                                           +--------+
        //
        // Convert sspi-rs security buffers to the Windows security buffers
        let mut sec_buffers = Vec::with_capacity(server_output_buffers.len());
        for buffer in server_output_buffers {
            sec_buffers.push(SecBuffer {
                cbBuffer: buffer.buffer.len() as u32,
                BufferType: buffer.buffer_type.to_u32().unwrap(),
                pvBuffer: vec_into_raw_ptr(buffer.buffer) as *mut _,
            });
        }
        client_input_buffers.cBuffers = sec_buffers.len() as u32;
        client_input_buffers.pBuffers = vec_into_raw_ptr(sec_buffers);

        if client_status == OK && server_status.status == SecurityStatus::CompleteNeeded {
            println!("Final stage.");

            let status = server.complete_auth_token(&mut []).unwrap();
            println!("Server complete auth token status: {status:?}");

            println!("Authentication successful.");
            return client_security_context;
        }
    }
}

pub unsafe fn show_session_key(client_security_context: &mut SecHandle) {
    let mut session_key = SecPkgContext_SessionKey::default();

    let status = QueryContextAttributesW(
        client_security_context,
        SECPKG_ATTR_SESSION_KEY,
        &mut session_key as *mut SecPkgContext_SessionKey as *mut _,
    );

    if status != 0 {
        panic!("Can not query session key. Error code: {:0x?}", status);
    }

    println!(
        "Established session key: {:?}",
        from_raw_parts(
            session_key.SessionKey as *const u8,
            session_key.SessionKeyLength as usize
        )
    );
}
