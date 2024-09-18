use super::{cmdchecker, lyltools};
use std::io;
use std::io::Write;

fn split_str(buffer: &str) -> Vec<&str> {
    buffer.split_whitespace().collect()
}

pub fn start_cli() {
    println!("Layl Shell\nCompiled On {}\nThis program comes with absolutely NO warranty, Use at your own risk.\n\nCopyright the Winrevive Community\n", lyltools::whats_the_version());
    let mut buffer = String::new();
    loop {
        print!("-> ");
        io::stdout()
            .flush()
            .expect("Error Flushing Standard Output");
        buffer.clear();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error Reading Input");
        let str_data = split_str(buffer.as_str());
        cmdchecker::cmd_checker(str_data);
    }
}
