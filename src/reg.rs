mod createreg;
mod deletereg;
mod writereg;

use super::Error;

pub fn start_mreg(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: mreg [type] (args for all types)");
        return Ok(());
    }
    match data[1].to_lowercase().as_str() {
        "-d" => {
            deletereg::delreg(data)?;
        }
        "-w" => {
            writereg::writereg(data)?;
        }
        "-c" => {
            createreg::createreg(data)?;
        }
        _ => {
            eprintln!("incorrect type");
        }
    }
    Ok(())
}
