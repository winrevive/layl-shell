use std::{
    fs::{self, File},
    io::{self, Read, Write},
    time::Instant,
};

use super::Result;
use winapi::um::synchapi::Sleep;

// Honestly, I don't really know what this is supposed to do, shouldn't we incorporate process elevation?
pub fn check_if_perms() -> Result<()> {
    let file = File::create("C:\\Windows\\idk.txt");
    match file {
        Ok(_) => match fs::remove_file("C:\\Windows\\idk.txt") {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        },
        Err(e) => return Err(e.into()),
    }
}

pub fn pause_terminal() -> Result<()> {
    println!("Press any key to continue...");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    io::stdout().flush()?;
    Ok(())
}

pub fn whats_the_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn wait(data: Vec<&str>) -> Result<()> {
    if data.len() <= 1 {
        return Err("usage: wait [time][extension]".into());
    }

    let chars = data[1].chars().collect::<Vec<char>>();
    let mut non_numericc = 0;
    let mut result = String::new();

    for c in chars {
        if !c.is_numeric() && !c.is_whitespace() {
            result.push(c);
            non_numericc += 1;
            if non_numericc == 2 {
                break;
            }
        }
    }

    let time = match result.as_str() {
        "ms" => data[1].trim_end_matches("ms").parse::<u32>()?,
        "s" => data[1].trim_end_matches("s").parse::<u32>()? * 1000,
        "m" => data[1].trim_end_matches("m").parse::<u32>()? * 60000,
        "h" => data[1].trim_end_matches("h").parse::<u32>()? * 3600000,
        "" => data[1].parse::<u32>()?,
        _ => {
            return Err("Invalid Time Extension".into());
        }
    };

    let start = Instant::now();

    unsafe {
        Sleep(time);
    }

    let elapsed = start.elapsed();
    let (elapsed_time, elapsed_time_extension) = match result.as_str() {
        "ms" => (elapsed.as_millis(), "ms"),
        "s" => (elapsed.as_secs().into(), "s"),
        "m" => ((elapsed.as_secs() / 60).into(), "m"),
        "h" => ((elapsed.as_secs() / 3600).into(), "h"),
        "" => (elapsed.as_millis(), "ms"),
        _ => {
            return Err("Invalid Time Extension".into());
        }
    };

    println!("Specified time: {}{}", time, elapsed_time_extension);
    println!(
        "Actual time (according to Rust timing): {}{}",
        elapsed_time, elapsed_time_extension
    );

    Ok(())
}
