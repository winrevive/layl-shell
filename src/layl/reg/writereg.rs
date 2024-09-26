use super::{RegistryGuard, Result};
use std::{ffi::CString, ptr};
use winapi::{
    shared::minwindef::{DWORD, HKEY__},
    um::{
        winnt::{KEY_SET_VALUE, REG_DWORD, REG_EXPAND_SZ, REG_QWORD, REG_SZ},
        winreg::{
            RegCloseKey, RegOpenKeyExA, RegSetValueExA, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
            HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
        },
    },
};

fn convert_reg_types(dwtype: &str) -> Option<DWORD> {
    match dwtype.to_lowercase().as_str() {
        "dword" => {
            return Some(REG_DWORD);
        }
        "sz" => {
            return Some(REG_SZ);
        }
        "expand_sz" => {
            return Some(REG_EXPAND_SZ);
        }
        "qword" => {
            return Some(REG_QWORD);
        }
        _ => {
            println!("mreg: incorrect registry type");
            return None;
        }
    }
}

pub fn write_registry(data: Vec<&str>) -> Result<()> {
    if data.len() != 7 {
        println!("usage: mreg -w [hklm/hkcr/hkcu/hku/hkcc] [regpath] [valuename] [dwtype] [value]");
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
    let reg_path_cstring = CString::new(data[3])?;
    unsafe {
        let o_status = RegOpenKeyExA(
            hkey,
            reg_path_cstring.as_ptr(),
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
        let value_name_cstring = CString::new(data[4])?;
        let value_type = if let Some(dwtype) = convert_reg_types(data[5]) {
            dwtype
        } else {
            return Ok(());
        };
        let data_cstring = CString::new(data[6])?;
        let data_ptr = data_cstring.as_ptr() as *const u8;
        let data_len = data[6].len() as u32;
        let c_status = RegSetValueExA(
            closing,
            value_name_cstring.as_ptr(),
            0,
            value_type,
            data_ptr,
            data_len,
        );
        let _guard = RegistryGuard(closing);

        if c_status != 0 {
            eprintln!("mreg: couldn't write registry value");
        } else {
            println!("mreg: registry value written!");
        }
        Ok(())
    }
}
