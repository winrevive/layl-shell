
use create_process_w::Command;


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

