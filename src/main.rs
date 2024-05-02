mod cli;
mod freader;
mod cmdchecker;
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
