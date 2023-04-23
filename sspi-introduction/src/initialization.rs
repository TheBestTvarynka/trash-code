use std::ptr::null_mut;

use sspi::Ntlm;
use windows_sys::Win32::Security::Authentication::Identity::{
    EnumerateSecurityPackagesW, InitSecurityInterfaceW, SecurityFunctionTableW,
};

use crate::utils::c_wide_string_to_rs_string;

// Client side

pub fn show_security_packages() {
    unsafe {
        let mut number_of_packages = 0;
        let mut packages = null_mut();

        let result = EnumerateSecurityPackagesW(&mut number_of_packages, &mut packages);

        if result != 0 || packages.is_null() {
            panic!(
                "Can not enumerate security packages. Error code: {:0x?}",
                result
            );
        }

        println!("Number of packages: {}", number_of_packages);
        println!("-------------------------");

        for i in 0..number_of_packages {
            let package_info = packages.add(i as usize);
            println!("fCapabilities: {}", (*package_info).fCapabilities);
            println!("wVersion: {}", (*package_info).wVersion);
            println!("wRPCID: {}", (*package_info).wRPCID);
            println!("cbMaxToken: {}", (*package_info).cbMaxToken);
            println!("Name: {}", c_wide_string_to_rs_string((*package_info).Name));
            println!(
                "Comment: {}",
                c_wide_string_to_rs_string((*package_info).Comment)
            );

            println!("-------------------------");
        }
    }
}

pub unsafe fn init_sspi_func_table() -> *mut SecurityFunctionTableW {
    InitSecurityInterfaceW()
}

// Server side

pub fn initialize_ntlm_server() -> Ntlm {
    Ntlm::new()
}
