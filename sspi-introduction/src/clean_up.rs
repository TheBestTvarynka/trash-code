use winapi::um::sspi::{DeleteSecurityContext, FreeCredentialsHandle, SecHandle};

/// Frees client credentials handle and deletes client security context
pub unsafe fn close_handles(
    client_cred_handle: &mut SecHandle,
    client_context_handle: *mut SecHandle,
) {
    let status = FreeCredentialsHandle(client_cred_handle);
    println!("FreeCredentialsHandle status: {:0x?}", status);

    let status = DeleteSecurityContext(client_context_handle);
    println!("DeleteSecurityContext status: {:0x?}", status);
}
