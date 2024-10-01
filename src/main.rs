mod layl;

use layl::{file_interpret::read_file, init, shell};
use std::{env, path::Path};

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        //color::write_color(Color::Blue);
        match init::read_init() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        match shell::start_shell() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        let filename = Path::new(&arg[1]);
        if filename.exists() {
            match read_file(filename) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        } else {
            eprintln!("File does not exist");
        }
    }
}
