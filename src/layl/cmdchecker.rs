use super::{color, io, power, processmgmt, reg, utils, Result};
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
            io::touch(data)?;
        }
        "fread" | "cat" => {
            io::cat(data)?;
        }
        "del" | "rm" => {
            io::layl_rm(data)?;
        }
        "copy" | "cpy" => {
            io::layl_copy(data)?;
        }
        "dcreate" | "mkdir" => {
            io::mkdir(data)?;
        }
        "cd" => {
            io::change_directory(data)?;
        }
        "pd" | "ls" => {
            io::print_directory(data)?;
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
        _ => match Command::new(data[0..].join(" ").as_str()).status() {
            Ok(_) => {}
            Err(e) => {
                if e.code() == 2 {
                    println!("{}: command not found", data[0]);
                } else {
                    eprintln!("{}", e);
                }
            }
        },
    }
    Ok(())
}
