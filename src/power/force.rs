use winapi::um::{powrprof::SetSuspendState, reason::SHTDN_REASON_MAJOR_SYSTEM, winuser::{ExitWindowsEx, EWX_FORCE, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN}};



pub fn force_power_procedures(data: Vec<&str>){
    match data[1].to_lowercase().as_str() {
        "-r" => {
            unsafe {
                let status = ExitWindowsEx(EWX_REBOOT | EWX_FORCE, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Restarting The Machine");
                    return;
                }
            }
        }
        "-s" => {
            unsafe {
                let status = ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Shutting Down The Machine");
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