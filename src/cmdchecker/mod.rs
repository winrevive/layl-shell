use std::process;
use crate::fileio;
use crate::dirio;

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
        _ => 
            println!("{} wasn't found", data[0])
    }
}
