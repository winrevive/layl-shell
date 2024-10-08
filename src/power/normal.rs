use winapi::um::{powrprof::SetSuspendState, reason::SHTDN_REASON_MAJOR_SYSTEM, winuser::{ExitWindowsEx, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN}};


pub fn normal_power_procedures(data: Vec<&str>){
    match data[1].to_lowercase().as_str() {
        "-r" => {
            unsafe {
                let status = ExitWindowsEx(EWX_REBOOT, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Restarting The Machine");
                    return;
                }
            }
        }
        "-s" => {
            unsafe {
                let status = ExitWindowsEx(EWX_SHUTDOWN, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Shutting Down The Machine");
                    return;
                }
            }
        }
        "-l" => {
            unsafe {
                let status = ExitWindowsEx(EWX_LOGOFF, SHTDN_REASON_MAJOR_SYSTEM);
                if status == 0 {
                    println!("Failed Logging off");
                    return;
                }
            }
        }
        "-h" => {
            unsafe {
                let status = SetSuspendState(1, 0, 0);
                if status == 0 {
                    println!("Failed Hibernating The Machine");
                    return;
                }
            }
        }
        "-q" => {
            unsafe {
                let status = SetSuspendState(0, 0, 0);
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