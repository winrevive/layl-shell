mod cli;
mod freader;
mod cmdchecker;
mod fileio;
mod dirio;
mod processmgmt;
mod lyltools;
use std::env;


fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        cli::start_cli();
    }
    else {
        freader::read_file(&arg[1]);
    }
}
