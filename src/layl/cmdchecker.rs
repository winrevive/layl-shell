use super::{color, dirio, fileio, power, processmgmt, reg, utils, Result};
use create_process_w::Command;
use std::process;

pub fn cmd_checker(data: Vec<&str>, buffer: &String) -> Result<()> {
    if data.is_empty() {
        return Ok(());
    }

    // TODO: Add inline environment; e.g FOO=bar echo $FOO

    match data[0].to_lowercase().as_str() {
        "echo" => {
            if data.len() <= 1 {
                println!("usage: echo [text]");
                return Ok(());
            }
            let output = &buffer[5..];
            println!("{} ", output);
        }
        "exit" => {
            process::exit(0);
        }
        "fcreate" | "touch" => {
            fileio::touch(data)?;
        }
        /*"fwrite" => {
            fileio::fwrite(data)?;
        }*/
        "fread" | "cat" => {
            fileio::cat(data)?;
        }
        "fdel" => {
            fileio::fdel(data)?;
        }
        "fcopy" => {
            fileio::fcopy(data)?;
        }
        /*"fprint" => {
            fileio::fprint(data)?;
        }*/
        "dcreate" | "mkdir" => {
            dirio::mkdir(data)?;
        }
        "ddel" => {
            dirio::ddel(data)?;
        }
        "cd" => {
            dirio::change_directory(data)?;
        }
        "pd" | "ls" => {
            dirio::print_directory(data)?;
        }
        "kill" => {
            processmgmt::kill_process(data)?;
        }
        "getpid" => {
            processmgmt::get_pid(data)?;
        }
        "sudo" => {
            processmgmt::sudo(data)?;
        }
        "mreg" => {
            reg::mreg(data)?;
        }
        "cls" | "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        "pause" => {
            utils::pause_terminal()?;
        }
        "power" => {
            power::power_management(data)?;
        }
        "color" => {
            color::color_chooser(data)?;
        }
        "about" => {
            println!("layl-shell, v{}", utils::whats_the_version());
        }
        "wait" => {
            utils::wait(data)?;
        }
        _ => {
            Command::new(data[0..].join(" ").as_str()).status()?;
        }
    }
    Ok(())
}
