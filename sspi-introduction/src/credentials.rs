use std::ptr::null_mut;

use sspi::{AcquireCredentialsHandleResult, AuthIdentity, AuthIdentityBuffers};
use winapi::{
    shared::rpcdce::SEC_WINNT_AUTH_IDENTITY_W,
    um::sspi::{
        AcquireCredentialsHandleW, CredHandle, QueryCredentialsAttributesW,
        SecPkgCredentials_NamesW, TimeStamp, SECPKG_CRED_ATTR_NAMES,
    },
};

use crate::utils::{c_wide_string_to_rs_string, str_to_win_wstring};

/// Acquires client credentials handle
pub unsafe fn acquire_client_credentials_handle(auth_data: &AuthIdentity) -> CredHandle {
    let mut credentials_handle = CredHandle::default();
    let mut expiry = TimeStamp::default();

    let mut package_name = str_to_win_wstring("NTLM");

    let domain_str = auth_data.domain.clone().unwrap_or_default();
    let mut user = str_to_win_wstring(auth_data.username.as_str());
    let mut domain = str_to_win_wstring(domain_str.as_str());
    let mut password = str_to_win_wstring(auth_data.password.as_ref());

    let mut identity = SEC_WINNT_AUTH_IDENTITY_W {
        User: user.as_mut_ptr(),
        UserLength: auth_data.username.len() as u32,
        Domain: domain.as_mut_ptr(),
        DomainLength: domain_str.len() as u32,
        Password: password.as_mut_ptr(),
        PasswordLength: auth_data.password.as_ref().len() as u32,
        Flags: 0x2,
    };

    let result = AcquireCredentialsHandleW(
        null_mut(),
        package_name.as_mut_ptr(),
        // SECPKG_CRED_OUTBOUND: https://learn.microsoft.com/en-us/windows/win32/secauthn/acquirecredentialshandle--ntlm
        2,
        null_mut(),
        &mut identity as *mut SEC_WINNT_AUTH_IDENTITY_W as *mut _,
        None,
        null_mut(),
        &mut credentials_handle as *mut _,
        &mut expiry as *mut _,
    );

    if result != 0 {
        panic!(
            "Can not acquire client credentials handle. Error code: {:0x?}",
            result,
        );
    }

    println!(
        "Client credentials handle: {} {}.",
        credentials_handle.dwLower, credentials_handle.dwUpper,
    );

    credentials_handle
}

/// Acquires server credentials handle
pub fn acquire_server_creds_handle(
    auth_data: &AuthIdentity,
) -> AcquireCredentialsHandleResult<Option<AuthIdentityBuffers>> {
    AcquireCredentialsHandleResult {
        credentials_handle: Some(auth_data.clone().into()),
        expiry: None,
    }
}

pub unsafe fn show_cred_info(client_credentials_handle: &mut CredHandle) {
    let mut credentials_name = SecPkgCredentials_NamesW::default();
    let status = QueryCredentialsAttributesW(
        client_credentials_handle,
        SECPKG_CRED_ATTR_NAMES,
        &mut credentials_name as *mut SecPkgCredentials_NamesW as *mut _,
    );

    if status != 0 {
        panic!("Can not query credentials name. Error code: {:0x?}", status);
    }

    println!(
        "Credentials name: {:?}",
        c_wide_string_to_rs_string(credentials_name.sUserName)
    );
}
