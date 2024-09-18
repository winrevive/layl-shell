mod powerperms;
mod procedures;

use super::Error;

pub fn power_management(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: power [type] [isForce]");
        return Ok(());
    }
    if powerperms::give_power_permissions() == false {
        println!("Failed Giving Permission To Get Power Control Of Your Computer");
        return Err("Failed Giving Permission To Get Power Control Of Your Computer".into());
    }
    procedures::power_procedures(data)?;
    Ok(())
}
