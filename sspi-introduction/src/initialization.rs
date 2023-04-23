use sspi::Ntlm;

// Client side

pub unsafe fn init_sspi_func_table() -> 

// Server side
pub fn initialize_ntlm_server() -> Ntlm {
    Ntlm::new()
}