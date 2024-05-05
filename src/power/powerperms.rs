use std::ptr;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winnt::{HANDLE, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, SE_PRIVILEGE_ENABLED};
use winapi::um::winbase::LookupPrivilegeValueA;
use winapi::um::winnt::TOKEN_PRIVILEGES;


pub fn give_power_permissions() -> bool {
    unsafe {
        let mut h_token: HANDLE = ptr::null_mut();
        let mut tkp: TOKEN_PRIVILEGES = std::mem::zeroed();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut h_token) == 0 {
            return false;
        }
        if LookupPrivilegeValueA(ptr::null_mut(), b"SeShutdownPrivilege\0".as_ptr() as _, &mut tkp.Privileges[0].Luid) == 0 {
            CloseHandle(h_token);
            return false;
        }
        tkp.PrivilegeCount = 1;
        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
        if AdjustTokenPrivileges(h_token, 0, &mut tkp, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
            CloseHandle(h_token);
            return false;
        }
        CloseHandle(h_token);
    }
    return true;
}