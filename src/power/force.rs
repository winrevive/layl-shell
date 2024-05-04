use phnt::ffi::NtShutdownSystem;
use winapi::{shared::ntdef::NT_SUCCESS, um::{powrprof::SetSuspendState, reason::SHTDN_REASON_MAJOR_SYSTEM, winuser::{ExitWindowsEx, EWX_LOGOFF}}};



pub fn force_power_procedures(data: Vec<&str>){
    match data[1].to_lowercase().as_str() {
        "-r" => {
            unsafe {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownReboot);
                if !NT_SUCCESS(status) {
                    println!("Failed Restarting The Machine, Error Code {}", status);
                    return;
                }
            }
        }
        "-s" => {
            unsafe {
                let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownPowerOff);
                if !NT_SUCCESS(status) {
                    println!("Failed Shutting Down The Machine, Error Code {}", status);
                    return;
                }
            }
        }
        "-l" => {
            unsafe {
                // No Force Option
                let status = ExitWindowsEx(EWX_LOGOFF, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Logging Ofg");
                    return;
                }
            }
        }
        "-h" => {
            unsafe {
                let status = SetSuspendState(1, 1, 0);
                if status == 0 {
                    println!("Failed Hibernating The Machine");
                    return;
                }
            }
        }
        "-q" => {
            unsafe {
                let status = SetSuspendState(0, 1, 0);
                if status == 0 {
                    println!("Failed Suspending The Machine");
                    return;
                }
            }
        }
        _ => {
            println!("Incorrect Type.");
            return;
        }
    }
}