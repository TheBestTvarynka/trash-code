use std::ptr::null_mut;

use sspi::Ntlm;
use winapi::um::sspi::QuerySecurityPackageInfoW;
use windows_sys::Win32::Security::Authentication::Identity::{
    EnumerateSecurityPackagesW, InitSecurityInterfaceW, SecurityFunctionTableW,
};

use crate::utils::{c_wide_string_to_rs_string, str_to_win_wstring};

/// Prints all available security packages on this machine
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

pub fn initialize_ntlm_server() -> Ntlm {
    Ntlm::new()
}

pub unsafe fn show_package_info(package_name: &str) {
    let mut raw_package_name = str_to_win_wstring(package_name);
    let mut package_info = null_mut();

    let status = QuerySecurityPackageInfoW(raw_package_name.as_mut_ptr(), &mut package_info);

    if status != 0 {
        panic!(
            "Can not query {} package info. Error code: {:0x?}",
            package_name, status
        );
    }

    println!("{} package info:", package_name);
    println!("fCapabilities: {}", (*package_info).fCapabilities);
    println!("wVersion: {}", (*package_info).wVersion);
    println!("wRPCID: {}", (*package_info).wRPCID);
    println!("cbMaxToken: {}", (*package_info).cbMaxToken);
    println!("Name: {}", c_wide_string_to_rs_string((*package_info).Name));
    println!(
        "Comment: {}",
        c_wide_string_to_rs_string((*package_info).Comment)
    );
}
