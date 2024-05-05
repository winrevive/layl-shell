
use std::{fs::{self, File}, io::{self, Read, Write}};

use create_process_w::Command;
use winapi::um::synchapi::Sleep;


pub fn start_process(process_string: &str) -> bool {
    let command = Command::new(process_string)
    .status();
    match command {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            //eprintln!("{}", e);
            return false;
        }
    }
}

pub fn check_if_perms() -> bool {
    let file = File::create("C:\\Windows\\idk.txt");
    match file {
        Ok(_) => {
            match fs::remove_file("C:\\Windows\\idk.txt") {
                Ok(_) => {
                    return true;
                }
                Err(_) => {
                    return true;
                }
            }
        }
        Err(_) => {
            return false;  
        }
    }
   
}

pub fn pause_terminal() {
    println!("Press Any Key To Continue To Layl-Shell....");  
    let mut buffer = [0; 1];   
    let _ = io::stdin().read_exact(&mut buffer);
    let _ = io::stdin().read(&mut buffer);
    io::stdout().flush().expect("Error Flushing Standard Output");
}

pub fn whats_the_version() -> &'static str {
    let ver = "May 5th 2024";
    ver
}

pub fn wait(data: Vec<&str>) {
    if data.len() <= 1 {
        println!("usage: wait [ms]");
        return;
    }
    let time: Result<u32, _> = data[1].parse();
    match time {
        Ok(n) => {
            unsafe{Sleep(n);}
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}