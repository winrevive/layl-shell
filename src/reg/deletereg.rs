use std::ptr;
use alloc::ffi::CString;
use winapi::{shared::minwindef::HKEY__, um::{winnt::KEY_SET_VALUE, winreg::{RegCloseKey, RegDeleteKeyA, RegOpenKeyExA, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS}}};

extern crate alloc;

pub fn delete_reg(data: Vec<&str>){
    if data.len() <= 3 {
        println!("usage: mreg -d [hkey] [regpath]");
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
        let d_status = RegDeleteKeyA(hkey, CString::new(data[3]).expect("Failed To Convert To CString").into_raw());
        if d_status != 0 {
            println!("Couldn't Delete Registry Key");
            if !closing.is_null() {
                RegCloseKey(closing);
            }
            return;
        }
    }
}