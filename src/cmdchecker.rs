use super::{dirio, fileio, power, processmgmt, reg, utils, Error};
use std::process;

pub fn cmd_checker(data: Vec<&str>) -> Result<(), Error> {
    if data.is_empty() {
        return Ok(());
    }
    match data[0].to_lowercase().as_str() {
        "echo" => {
            if data.len() <= 1 {
                return Ok(());
            }
            for arg in &data[1..] {
                print!("{} ", arg);
            }
            print!("\n");
        }
        "exit" => {
            process::exit(0);
        }
        "touch" => {
            fileio::touch(data)?;
        }
        "fdel" => {
            fileio::fdel(data)?;
        }
        "fcopy" => {
            fileio::fcopy(data)?;
        }
        "mkdir" => {
            dirio::mkdir(data)?;
        }
        "ddel" => {
            dirio::deldir(data)?;
        }
        "cd" => {
            dirio::change_directory(data)?;
        }
        "pd" => {
            dirio::print_directory(data)?;
        }
        "kill" => {
            processmgmt::kill_process(data)?;
        }
        "getpid" => processmgmt::pid(data)?,
        "sudo" => {
            processmgmt::sudo(data)?;
        }
        "modreg" => {
            if utils::check_if_perms() == true {
                reg::start_mreg(data)?;
            } else {
                eprintln!("This Command Requires Administrative Privliages, Please Run Layl As Administrator.");
                return Err("This Command Requires Administrative Privliages".into());
            }
        }
        "cls" | "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        "pause" => utils::pause_terminal()?,
        "power" => {
            power::power_management(data)?;
        }
        "about" => {
            println!("layl-shell v{}", utils::whats_the_version());
        }
        "wait" => {
            utils::wait(data)?;
        }
        _ => {
            utils::start_process(data[0..].join(" ").as_str())?;
        }
    }
    Ok(())
}
