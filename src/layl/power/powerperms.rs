use super::Result;
use std::ptr;
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{GetCurrentProcess, OpenProcessToken},
    securitybaseapi::AdjustTokenPrivileges,
    winbase::LookupPrivilegeValueA,
    winnt::{HANDLE, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY},
};

pub fn give_power_permissions() -> Result<()> {
    unsafe {
        let mut h_token: HANDLE = ptr::null_mut();
        let mut tkp: TOKEN_PRIVILEGES = std::mem::zeroed();
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut h_token,
        ) == 0
        {
            return Err("Failed to open process token".into());
        }
        if LookupPrivilegeValueA(
            ptr::null_mut(),
            b"SeShutdownPrivilege\0".as_ptr() as _,
            &mut tkp.Privileges[0].Luid,
        ) == 0
        {
            CloseHandle(h_token);
            return Err("Failed to lookup privilege value".into());
        }
        tkp.PrivilegeCount = 1;
        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
        if AdjustTokenPrivileges(h_token, 0, &mut tkp, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
            CloseHandle(h_token);
            return Err("Failed to adjust token privileges".into());
        }
        CloseHandle(h_token);
    }
    Ok(())
}
