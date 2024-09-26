use super::Result;
use phnt::ffi::NtShutdownSystem;
use winapi::{
    shared::ntdef::NT_SUCCESS,
    um::{
        powrprof::SetSuspendState,
        reason::SHTDN_REASON_MAJOR_SYSTEM,
        winuser::{ExitWindowsEx, LockWorkStation, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN},
    },
};

pub fn power_procedures(data: Vec<&str>) -> Result<()> {
    match data[1].to_lowercase().as_str() {
        "-r" => unsafe {
            if ["-f", "--force"].contains(&data[2]) {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownReboot);
                if !NT_SUCCESS(status) {
                    return Err("Failed to forcefully restart system".into());
                }
            } else {
                let status = ExitWindowsEx(EWX_REBOOT, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    return Err("Failed to restart system".into());
                }
            }
        },
        "-s" => unsafe {
            if ["-f", "--force"].contains(&data[2]) {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownPowerOff);
                if !NT_SUCCESS(status) {
                    return Err("Failed to forcefully shutdown system".into());
                }
            } else {
                let status = ExitWindowsEx(EWX_SHUTDOWN, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    return Err("Failed to shutdown system".into());
                }
            }
        },
        "-l" => unsafe {
            let status = ExitWindowsEx(EWX_LOGOFF, SHTDN_REASON_MAJOR_SYSTEM);
            if status == 0 {
                return Err("Failed to log off".into());
            }
        },
        "-lk" => unsafe {
            let status = LockWorkStation();
            if status == 0 {
                return Err("Failed to lock system".into());
            }
        },
        "-h" => unsafe {
            let status = SetSuspendState(1, 0, 0);
            if status == 0 {
                return Err("Failed to hibernate system".into());
            }
        },
        "-q" => unsafe {
            let status = SetSuspendState(0, 0, 0);
            if status == 0 {
                return Err("Failed to suspend system".into());
            }
        },
        _ => {
            println!("Incorrect Type.");
            return Ok(());
        }
    }
    Ok(())
}
