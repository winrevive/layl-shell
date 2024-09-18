use winapi::um::{
    powrprof::SetSuspendState,
    reason::SHTDN_REASON_MAJOR_SYSTEM,
    winuser::{ExitWindowsEx, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN},
};

use phnt::ffi::NtShutdownSystem;

use super::Error;

pub fn power_procedures(data: Vec<&str>) -> Result<(), Error> {
    match data[1].to_lowercase().as_str() {
        "-r" => unsafe {
            if ["--force", "-f"].contains(&data[2]) {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownReboot);
                if status != 0 {
                    println!("Failed Restarting The Machine");
                    return Err("Failed Restarting The Machine".into());
                }
            } else {
                let status = ExitWindowsEx(EWX_REBOOT, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Restarting The Machine");
                    return Err("Failed Restarting The Machine".into());
                }
            }
        },
        "-s" => unsafe {
            if ["--force", "-f"].contains(&data[2]) {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownPowerOff);
                if status != 0 {
                    println!("Failed Shutting Down The Machine");
                    return Err("Failed Shutting Down The Machine".into());
                }
            } else {
                let status = ExitWindowsEx(EWX_SHUTDOWN, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Shutting Down The Machine");
                    return Err("Failed Shutting Down The Machine".into());
                }
            }
        },
        "-l" => unsafe {
            let status = ExitWindowsEx(EWX_LOGOFF, SHTDN_REASON_MAJOR_SYSTEM);
            if status == 0 {
                println!("Failed Logging off");
                return Err("Failed Logging off".into());
            }
        },
        "-h" => unsafe {
            let status = SetSuspendState(1, 0, 0);
            if status == 0 {
                println!("Failed Hibernating The Machine");
                return Err("Failed Hibernating The Machine".into());
            }
        },
        "-q" => unsafe {
            let status = SetSuspendState(0, 0, 0);
            if status == 0 {
                println!("Failed Suspending The Machine");
                return Err("Failed Suspending The Machine".into());
            }
        },
        _ => {
            println!("Incorrect Type.");
            return Err("Incorrect Type.".into());
        }
    }
    Ok(())
}
