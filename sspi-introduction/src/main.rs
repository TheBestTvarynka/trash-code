mod authentication;
mod credentials;
mod initialization;
mod messages_communication;
mod utils;

use sspi::AuthIdentity;

use crate::{
    credentials::{acquire_client_credentials_handle, acquire_server_creds_handle},
    initialization::{init_sspi_func_table, initialize_ntlm_server, show_security_packages},
};

fn main() {
    println!("SSPI NTLM authentication example.");

    show_security_packages();

    println!("Initialization...");

    let client = unsafe { init_sspi_func_table() };
    let mut server = initialize_ntlm_server();

    println!("Set up credentials...");

    // Credentials from the local computer user.
    // I already deleted this user so don't worry :)
    let credentials = AuthIdentity {
        username: "testuser".into(),
        password: "test".to_owned().into(),
        domain: None, // we don't need a domain for the NTLM
    };

    let client_credentials_handle = unsafe { acquire_client_credentials_handle(&credentials) };

    let server_credentials_handle = acquire_server_creds_handle(&credentials);

    println!("Authentication...");

    todo!();

    println!("Message communication...");
}
