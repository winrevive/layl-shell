use super::Result;
use std::{ffi::CString, ptr};
use winapi::{
    shared::minwindef::HKEY__,
    um::{
        winnt::KEY_SET_VALUE,
        winreg::{
            RegCloseKey, RegDeleteKeyA, RegOpenKeyExA, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
            HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
        },
    },
};

pub fn delete_reg(data: Vec<&str>) -> Result<()> {
    if data.len() <= 3 {
        println!("usage: mreg -d [hklm/hkcr/hkcu/hku/hkcc] [regpath]");
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
            println!("mreg: incorrect key value");
            return Ok(());
        }
    };
    unsafe {
        let o_status = RegOpenKeyExA(
            hkey,
            CString::new(data[3])?.into_raw(),
            0,
            KEY_SET_VALUE,
            &mut closing,
        );
        if o_status != 0 {
            println!("mreg: couldn't find registry key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return Ok(());
        }
        let d_status = RegDeleteKeyA(hkey, CString::new(data[3])?.into_raw());
        if d_status != 0 {
            println!("mreg: couldn't delete registry key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
        } else {
            println!("mreg: registry key deleted!");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
        }
    }
    Ok(())
}
