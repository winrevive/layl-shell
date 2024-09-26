use std::io::{self, BufRead};
use std::fs::File;
use crate::cmdchecker;

fn split_str(buffer: &str) -> Vec<&str>{
    buffer.split_whitespace().collect()
}

pub fn read_file(filename: &str){
    let file = File::open(filename);
    match file {
        Ok(file) =>{
            let reader = io::BufReader::new(file);
            for line in reader.lines(){
                match line {
                    Ok(line) => {
                        let splitted_line = split_str(&line);
                        cmdchecker::cmd_checker(splitted_line, line.clone());
                    }
                    Err(_) => {
                        println!("");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error, {}", e);
        }
    }
}