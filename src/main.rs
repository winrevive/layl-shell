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
mod color;

//use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        //color::write_color(Color::Blue);
        init::read_init();
        cli::start_cli();
    }
    else {
        freader::read_file(&arg[1]);
    }
}
