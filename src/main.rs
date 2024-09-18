mod cli;
mod cmdchecker;
mod dirio;
mod fileio;
mod freader;
mod lyltools;
mod power;
mod processmgmt;
mod reg;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        cli::start_cli();
    } else {
        freader::read_file(&arg[1]);
    }
}
