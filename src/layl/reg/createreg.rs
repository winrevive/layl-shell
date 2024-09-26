use super::Result;
use std::{ffi::CString, ptr};
use winapi::{
    shared::minwindef::HKEY__,
    um::{
        winnt::KEY_READ,
        winreg::{
            RegCloseKey, RegCreateKeyA, RegOpenKeyExA, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
            HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
        },
    },
};

pub fn create_registry(data: Vec<&str>) -> Result<()> {
    if data.len() <= 3 {
        println!("usage: mreg -c [hklm/hkcr/hkcu/hku/hkcc] [key]");
        return Ok(());
    }
    let mut closing: *mut HKEY__ = ptr::null_mut();
    let hkey = match data[2].to_lowercase().as_str() {
        "hklm" => HKEY_LOCAL_MACHINE,
        "hkcr" => HKEY_CLASSES_ROOT,
        "hkcu" => HKEY_CURRENT_USER,
        "hku" => HKEY_USERS,
        "hkcc" => HKEY_CURRENT_CONFIG,
        _ => {
            // More Keys Will Be Added Soon!
            eprintln!("mreg: incorrect key value");
            return Ok(());
        }
    };
    unsafe {
        let status = RegOpenKeyExA(
            hkey,
            CString::new(data[3])?.into_raw(),
            0,
            KEY_READ,
            &mut closing,
        );
        if status == 0 {
            println!("mreg: registry key already exists");
            RegCloseKey(closing);
            return Ok(());
        }
        let c_status = RegCreateKeyA(hkey, CString::new(data[3])?.into_raw(), &mut closing);
        if c_status == 0 {
            println!("mreg: registry key created!");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
        } else {
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            eprintln!(
                "mreg: failed creating registry key, error code: {}",
                c_status
            );
        }
    }
    Ok(())
}
