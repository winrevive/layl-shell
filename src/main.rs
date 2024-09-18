mod cmdchecker;
mod dirio;
mod fileio;
mod freader;
mod power;
mod processmgmt;
mod reg;
mod shell;
mod utils;
use std::env;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let arg: Vec<String> = env::args().collect();
    if arg.len() <= 1 {
        shell::start_shell()?;
    } else {
        freader::interpret_file(&arg[1])?;
    }
    Ok(())
}
