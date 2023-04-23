mod authentication;
mod credentials;
mod initialization;
mod messages_communication;

use sspi::{AuthIdentity, Ntlm};

fn main() {
    println!("SSPI NTLM authentication example");

    let credentials = AuthIdentity {
        username: "test_user".into(),
        password: "".to_owned().into(),
        domain: None, // we don't need a domain for the NTLM
    };

    // initialize NTLM server
    let ntlm_server = Ntlm::new();
}

fn authenticate() {

}
