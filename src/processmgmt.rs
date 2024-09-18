use super::Error;
use std::{ffi::CString, ptr};
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{OpenProcess, TerminateProcess},
    shellapi::ShellExecuteA,
    winnt::PROCESS_TERMINATE,
    winuser::{FindWindowA, GetWindowThreadProcessId},
};

fn get_pid(process: &str) -> Result<u32, Error> {
    let process_cstr = CString::new(process)?;
    let mut pid: u32 = 0;
    let hwnd = unsafe { FindWindowA(ptr::null_mut(), process_cstr.as_ptr()) };
    if hwnd == ptr::null_mut() {
        println!("Failed Getting PID to process");
        return Err("Failed Getting PID to process".into());
    }
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };

    Ok(pid)
}

pub fn kill_process(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: kill [pid of process or process name]");
        return Ok(());
    }

    let pid: u32;

    if data[1].parse::<u32>().is_err() {
        pid = get_pid(data[1])?;
    } else {
        pid = data[1].parse()?;
    }

    let process_handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
    if process_handle != ptr::null_mut() {
        let result = unsafe { TerminateProcess(process_handle, 0) };
        if result == 0 {
            //let error_code = unsafe { GetLastError() };
            println!("Failed to terminate process");
            unsafe { CloseHandle(process_handle) };
            return Err("Failed to terminate process".into());
        }
        unsafe { CloseHandle(process_handle) };
    }
    Ok(())
}

pub fn pid(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: getpid [process-name]");
        return Ok(());
    }
    println!("PID: {}", get_pid(data[1])?);
    Ok(())
}

pub fn sudo(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: sudo [command]");
        return Ok(());
    }
    let types = CString::new("runas")?;
    let command = CString::new(data[1])?;
    let args = CString::new(data[2..].join(" "))?;
    unsafe {
        ShellExecuteA(
            ptr::null_mut(),
            types.as_ptr(),
            command.as_ptr(),
            args.as_ptr(),
            ptr::null_mut(),
            5,
        );
    }
    Ok(())
}
