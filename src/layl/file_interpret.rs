use super::{cmdchecker, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_file(filename: &Path) -> Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        cmdchecker::cmd_checker(line.split_whitespace().collect(), &line)?;
    }
    Ok(())
}
