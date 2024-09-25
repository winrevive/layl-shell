use std::process;


use crate::fileio;
use crate::dirio;
use crate::power;
use crate::processmgmt;
use crate::lyltools;
use crate::reg;
use crate::color;

pub fn cmd_checker(data: Vec<&str>){
    if data.is_empty() {
        return;
    }
    match data[0].to_lowercase().as_str() {
        "echo" => {
            if data.len() <= 1 {
                return;
            }
            for arg in &data[1..] {
                print!("{} ", arg);
            }
            println!(); 
        }
        "exit" => {
            process::exit(0);
        }
        "fcreate" => {
            fileio::fcreate(data);
        }
        "fwrite" => {
            fileio::fwrite(data);
        }
        "fread" => {
            fileio::fread(data);
        }
        "fdelete" => {
            fileio::fdelete(data);
        }
        "fcopy" => {
            fileio::fcopy(data);
        }
        "fprint" => {
            fileio::fprint(data)
        }
        "dcreate" => {
            dirio::dcreate(data);
        }
        "ddelete" => {
            dirio::ddelete(data);
        }
        "rddelete" => {
            dirio::rddelete(data);
        }
        "moveto" => {
            dirio::change_directory(data);
        }
        "cd" => {
            dirio::current_directory();
        }
        "pd" => {
            dirio::print_directory(data);
        }
        "kill" => {
            processmgmt::kill_process(data);
        }
        "getpid" => {
            processmgmt::get_pid(data)
        }
        "sudo" => {
            processmgmt::sudo(data);
        }
        "mreg" => {
            if lyltools::check_if_perms() == true {
                reg::start_mreg(data);
            }
            else {
                println!("This Command Requires Administrative Privliages, Please Run Layl As Administrator.")
            }
            
        }
        "cls" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        "pause" => {
            lyltools::pause_terminal()
        }
        "power" => {
            power::power_management(data);
        }
        "about" => {
            println!("Layl-Shell\nCompiled On {}",lyltools::whats_the_version());
        }
        "wait" => {
            lyltools::wait(data);
        }
        "color" => {
            color::color_chooser(data);
        }
        _ => {
            if lyltools::start_process(data[0..].join(" ").as_str()) == false {
                println!("{} wasn't found", data[0])
            }
        }
    }
}
