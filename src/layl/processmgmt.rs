use super::Result;
use std::{ffi::CString, ptr};
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{OpenProcess, TerminateProcess},
    shellapi::ShellExecuteA,
    winnt::PROCESS_TERMINATE,
    winuser::{FindWindowA, GetWindowThreadProcessId},
};

fn convert_process_name_to_pid(process_name: &str) -> Result<u32> {
    let mut pid: u32 = 0;
    let process_name = CString::new(process_name)?;
    let hwnd = unsafe { FindWindowA(ptr::null_mut(), process_name.as_ptr()) };
    if hwnd == ptr::null_mut() {
        return Err("Failed Getting PID to process".into());
    }
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    Ok(pid)
}

pub fn kill_process(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: kill [pid/process name]");
        return Ok(());
    }
    let pid: u32;

    let process = data[1..].join(" ");

    if process.parse::<u32>().is_ok() {
        pid = process.parse()?;
    } else {
        pid = convert_process_name_to_pid(&process)?;
    }

    let process_handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
    if process_handle != ptr::null_mut() {
        let result = unsafe { TerminateProcess(process_handle, 0) };
        if result == 0 {
            eprintln!("kill: Failed to terminate process");
            return Err("Failed to terminate process".into());
        }

        unsafe { CloseHandle(process_handle) };
    }
    Ok(())
}

pub fn get_pid(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: getpid [window-name]");
        return Ok(());
    }

    let process_name = data[1..].join(" ");
    let pid = convert_process_name_to_pid(&process_name)?;

    println!("get_pid: PID Is {}", pid);
    Ok(())
}

pub fn sudo(data: Vec<&str>) -> Result<()> {
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
