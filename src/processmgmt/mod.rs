use std::ptr;

use alloc::ffi::CString;
use winapi::um::{handleapi::CloseHandle, processthreadsapi::{OpenProcess, TerminateProcess}, shellapi::ShellExecuteA, winnt::PROCESS_TERMINATE, winuser::{FindWindowA, GetWindowThreadProcessId}};
extern crate winapi;
extern crate alloc;



pub fn kill_process(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: kill [pid]");
        return;
    }
    let pid: Result<u32, _> = data[1].parse();
    match pid {
        Ok(n) => {
            let process_handle = unsafe {OpenProcess(PROCESS_TERMINATE, 0, n)};
            if process_handle != ptr::null_mut() {
                let result = unsafe { TerminateProcess(process_handle, 0)};
                if result == 0 {
                    //let error_code = unsafe { GetLastError() };
                    println!("Failed to terminate process");
                }
                unsafe { CloseHandle(process_handle)};
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn get_pid(data: Vec<&str>){
    let mut pid:u32 = 0;
    if data.len() <= 1 {
        println!("usage: getpid [window-name]");
        return;
    }
    let window_name = CString::new(data[1..].join(" ")).map_err(|_| "Failed to convert window name to CString".to_string());    
    match window_name {
        Ok(n) => {
                let hwnd = unsafe{FindWindowA(ptr::null_mut(), n.as_ptr())};
                if hwnd == ptr::null_mut(){
                    println!("Failed Getting PID to process");
                    return;
                }
                unsafe {GetWindowThreadProcessId(hwnd, &mut pid)};
                println!("PID Is {}", pid);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}



pub fn sudo(data: Vec<&str>) {
    if data.len() <= 1 {
        println!("usage: getpid [window-name]");
        return;
    }
    let types = CString::new("runas").expect("Failed to convert to CString");
    let command = CString::new(data[1]).expect("Failed to convert to CString");
    let args = CString::new(data[2..].join(" ")).expect("Failed To Convert To CString");    
    unsafe {
        ShellExecuteA(
            ptr::null_mut(),
            types.as_ptr(),
            command.as_ptr(),
            args.as_ptr(),
            ptr::null_mut(),
            5
        );
    }
}



