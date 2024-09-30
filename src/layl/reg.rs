mod createreg;
mod deletereg;
mod writereg;

use super::{utils::check_if_perms, Result};
use winapi::{shared::minwindef::HKEY__, um::winreg::RegCloseKey};

pub struct RegistryGuard(*mut HKEY__);
impl Drop for RegistryGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            println!("mreg: closing registry key");
            unsafe {
                RegCloseKey(self.0);
            }
        }
    }
}

pub fn mreg(data: Vec<&str>) -> Result<()> {
    if check_if_perms().is_err() {
        println!("mreg: insufficient permissions");
        return Ok(());
    }

    if data.len() <= 1 {
        println!("usage: mreg [type] (args for all types)");
        return Ok(());
    }
    match data[1].to_lowercase().as_str() {
        "-d" => {
            deletereg::delete_reg(data)?;
        }
        "-w" => {
            writereg::write_registry(data)?;
        }
        "-c" => {
            createreg::create_registry(data)?;
        }
        _ => {
            println!("Incorrect type");
        }
    }
    Ok(())
}
