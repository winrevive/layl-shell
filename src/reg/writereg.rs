use std::ptr;

use alloc::ffi::CString;
use winapi::{shared::minwindef::{DWORD, HKEY__}, um::{winnt::{KEY_SET_VALUE, REG_DWORD, REG_EXPAND_SZ, REG_QWORD, REG_SZ}, winreg::{RegCloseKey, RegOpenKeyExA, RegSetValueExA, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS}}};


extern crate alloc;

fn convert_reg_types(dwtype: &str) -> DWORD{
    match dwtype.to_lowercase().as_str() {
        "dword" => {
            return REG_DWORD;
        }
        "sz" => {
            return REG_SZ;
        }
        "expand_sz" => {
            return REG_EXPAND_SZ;
        }
        "qword" => {
            return REG_QWORD;
        }
        _ => {
            println!("Incorrect Registry Type, Returning REG_SZ..");
            return REG_SZ;
        }
    }
}

pub fn write_registry(data: Vec<&str>){
    if data.len() <= 6 {
        println!("usage: mreg -w [hkey] [regpath] [valuename] [dwtype] [value]");
        return
    }
    let mut closing: *mut HKEY__ = ptr::null_mut();
    let hkey = match data[2].to_lowercase().as_str() {
        "hklm" => {
            HKEY_LOCAL_MACHINE
        }
        "hkcr" => {
            HKEY_CLASSES_ROOT
        }
        "hkcu" => {
            HKEY_CURRENT_USER
        }
        "hku" => {
            HKEY_USERS
        }
        "hkcc" => {
            HKEY_CURRENT_CONFIG
        }
        _ => {
            // More Keys Will Be Added Soon!
            println!("Incorrect Key Value");
            return;
        }
    };
    unsafe {
        let o_status = RegOpenKeyExA(hkey, CString::new(data[3]).expect("Failed To Convert To CString").into_raw(), 0, KEY_SET_VALUE, &mut closing);
        if o_status != 0 {
            println!("Couldn't Find Registry Key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return;
        }
        let value_name_cstring = CString::new(data[4]).expect("Failed To Convert To CString");
        let value_type = convert_reg_types(data[5]);
        let data_cstring = CString::new(data[6]).expect("Failed To Convert To CString");
        let data_ptr = data_cstring.as_ptr() as *const u8;
        let data_len = data[6].len() as u32;
        let c_status = RegSetValueExA(closing, value_name_cstring.as_ptr(), 0, value_type, data_ptr, data_len);
        if c_status != 0 {
            println!("Couldn't Write Registry Value");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return;
        }
    }
}