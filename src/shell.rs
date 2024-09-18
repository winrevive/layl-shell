use super::Error;
use super::{cmdchecker, utils};
use std::io;
use std::io::Write;

pub fn start_shell() -> Result<(), Error> {
    println!(
        "Layl Shell\n
    Compiled On {}\n
    This program comes with absolutely NO warranty, Use at your own risk.\n\n

    Copyright the Winrevive Community\n",
        utils::whats_the_version()
    );
    let mut buffer = String::new();
    loop {
        print!("-> ");
        io::stdout().flush()?;
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let str_data = buffer.split_whitespace().collect();
        cmdchecker::cmd_checker(str_data)?;
    }
}
