mod cli;
mod freader;
mod cmdchecker;
mod fileio;
mod dirio;
mod processmgmt;
mod lyltools;
mod reg;
mod power;
use std::env;
mod init;

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        init::read_init();
        cli::start_cli();
    }
    else {
        freader::read_file(&arg[1]);
    }
}
