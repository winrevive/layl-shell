mod layl;

use layl::{fileio, shell};
use std::env;

//use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        //color::write_color(Color::Blue);
        match fileio::read_file("C:\\programdata\\init.lyl") {
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
        match fileio::read_file(&arg[1]) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
