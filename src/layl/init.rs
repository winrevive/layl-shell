use std::{fs::File, path::Path};

use super::{file_interpret::read_file, Result};

// Init module seems useless for now.

pub fn read_init() -> Result<()> {
    let user_profile = std::env::var("USERPROFILE")?;
    let default_path = Path::new(&user_profile).join(".laylrc");

    if !default_path.exists() {
        File::create(&default_path)?;
        return Ok(());
    }

    read_file(&default_path)?;
    Ok(())
}
