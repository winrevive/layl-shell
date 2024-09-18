use super::Error;
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

pub fn createreg(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 3 {
        println!("usage: mreg -c [hkey] [regpath]");
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
        let status = RegOpenKeyExA(
            hkey,
            CString::new(data[3])
                .expect("Failed To Convert To CString")
                .into_raw(),
            0,
            KEY_READ,
            &mut closing,
        );
        if status == 0 {
            println!("Registry Key already exists");
            RegCloseKey(closing);
            return Err("Registry Key already exists".into());
        }
        let c_status = RegCreateKeyA(
            hkey,
            CString::new(data[3])
                .expect("Failed To Convert To CString")
                .into_raw(),
            &mut closing,
        );
        if c_status == 0 {
            println!("Registry Key Created!");
            RegCloseKey(closing);
            return Ok(());
        } else {
            RegCloseKey(closing);
            println!("Failed Making Registry Key, Error Code {}\n", c_status);
            return Err("Failed Making Registry Key".into());
        }
    }
}
