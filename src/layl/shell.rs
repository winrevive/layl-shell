use super::{
    cmdchecker,
    //utils::whats_the_version,
    Result,
};
use std::io::{stdin, stdout, Write};

// TODO: clean this up later

pub fn start_shell() -> Result<()> {
    //println!("Layl Shell\nCompiled On {}\nThis program comes with absolutely NO warranty, Use at your own risk.\n\nCopyright the Winrevive Community\n", whats_the_version());
    let mut buffer = String::new();
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!("-> ");
        stdout.flush()?;
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        cmdchecker::cmd_checker(buffer.split_whitespace().collect(), &buffer)?;
    }
}
