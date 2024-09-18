use super::Error;
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
pub fn delreg(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 3 {
        println!("usage: modreg -d [hkey] [regpath]");
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
            println!("Incorrect Key Value");
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
            println!("Couldn't Find Registry Key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return Err("Couldn't Find Registry Key".into());
        }
        let d_status = RegDeleteKeyA(hkey, CString::new(data[3])?.into_raw());
        if d_status != 0 {
            println!("Couldn't Delete Registry Key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return Err("Couldn't Delete Registry Key".into());
        }
    }
    Ok(())
}
