use super::{cmdchecker, Error};

use std::fs::File;
use std::io::{self, BufRead};

pub fn interpret_file(filename: &str) -> Result<(), Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        cmdchecker::cmd_checker(line.split_whitespace().collect())?;
    }
    Ok(())
}
