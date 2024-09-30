use super::{fileio, Result};

// Init module seems useless for now.

pub fn read_init() -> Result<()> {
    fileio::read_file("C:\\programdata\\init.lyl")?;
    Ok(())
}
