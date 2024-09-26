mod powerperms;
mod powerproc;

use super::Result;

pub fn power_management(data: Vec<&str>) -> Result<()> {
    if data.len() <= 1 {
        println!("usage: power [type] [-f / --force]");
        return Ok(());
    }
    powerperms::give_power_permissions()?;
    powerproc::power_procedures(data)?;
    Ok(())
}
