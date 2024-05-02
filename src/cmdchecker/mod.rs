use std::process;

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
        _ => 
            println!("{} wasn't found", data[0])
    }
}
