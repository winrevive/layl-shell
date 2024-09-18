use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
};

use super::Error;
use create_process_w::Command;
use winapi::um::synchapi::Sleep;

pub fn start_process(process_string: &str) -> Result<(), Error> {
    Command::new(process_string).status()?;
    Ok(())
}

pub fn check_if_perms() -> bool {
    let file = File::create("C:\\Windows\\idk.txt");
    match file {
        Ok(_) => match fs::remove_file("C:\\Windows\\idk.txt") {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        },
        Err(_) => {
            return false;
        }
    }
}

pub fn pause_terminal() -> Result<(), Error> {
    println!("Press any key to continue...");
    let mut buffer = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer);
    let _ = io::stdin().read(&mut buffer);
    io::stdout().flush()?;
    Ok(())
}

pub fn whats_the_version() -> &'static str {
    let ver = env!("CARGO_PKG_VERSION");
    ver
}

pub fn wait(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: wait [ms]");
        return Ok(());
    }
    let time: u32 = data[1].parse()?;
    unsafe {
        Sleep(time);
    }
    Ok(())
}
