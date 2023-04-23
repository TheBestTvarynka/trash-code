mod authentication;
mod clean_up;
mod credentials;
mod initialization;
mod signature;
mod utils;

use sspi::AuthIdentity;

use crate::{
    authentication::authenticate,
    clean_up::close_handles,
    credentials::{acquire_client_credentials_handle, acquire_server_creds_handle},
    initialization::{init_sspi_func_table, initialize_ntlm_server, show_security_packages},
    signature::client_make_signature,
};

fn main() {
    println!("SSPI NTLM authentication example.");

    show_security_packages();

    println!("Initialization...");

    // just an example. we don't need it for the rest of the program
    let _functions_table = unsafe { init_sspi_func_table() };

    let mut server = initialize_ntlm_server();

    println!("Set up credentials...");

    let credentials = AuthIdentity {
        username: "testuser".into(),
        password: "test".to_owned().into(),
        domain: None, // we don't need a domain for the NTLM
    };

    let mut client_credentials_handle = unsafe { acquire_client_credentials_handle(&credentials) };

    let server_credentials_handle = acquire_server_creds_handle(&credentials).credentials_handle;

    println!("Authentication...");

    let mut client_security_context = unsafe {
        authenticate(
            client_credentials_handle,
            server_credentials_handle,
            &mut server,
        )
    };

    println!("Communication...");

    unsafe {
        client_make_signature(b"TheBestTvarynka", &mut client_security_context);
    }

    println!("Finishing...");

    unsafe {
        close_handles(&mut client_credentials_handle, &mut client_security_context);
    }
}
