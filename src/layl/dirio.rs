use super::Result;
use std::{env, fs, path::Path};

pub fn mkdir(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: mkdir [dirname]");
        return Ok(());
    }
    fs::create_dir(data[1])?;
    println!("mkdir: created directory {}", data[1]);
    Ok(())
}

pub fn ddel(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: ddel [-r/--recursive] [dirname]");
        return Ok(());
    }

    if ["-r", "--recursive"].contains(&data[1]) && data.len() != 3 {
        println!("usage: ddel [-r/--recursive] [dirname]");
        return Ok(());
    } else if ["-r", "--recursive"].contains(&data[1]) {
        fs::remove_dir_all(data[2])?;
        println!("ddel: recursively deleted directory {}", data[2]);
        return Ok(());
    }

    fs::remove_dir(data[1])?;
    println!("ddel: deleted directory {}", data[1]);
    Ok(())
}

// pub fn dcopy(data: Vec<&str>){
//     // TODO: Add Recursive Directory Copying
// }

pub fn change_directory(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("cd: current directory: {}", env::current_dir()?.display());
        return Ok(());
    }

    let path = Path::new(data[1]);

    env::set_current_dir(path)?;
    println!("cd: changed directory to {}", path.display());

    Ok(())
}

pub fn print_directory(data: Vec<&str>) -> Result<()> {
    let path_str = env::current_dir()?.display().to_string();

    let str: &str = if data.len() <= 1 { &path_str } else { data[1] };

    let mut i: u32 = 0;
    let fs = fs::read_dir(str)?;
    for file in fs {
        let file = file?;

        let file_name = if let Ok(string) = file.file_name().into_string() {
            string
        } else {
            eprintln!("pd: failed to convert file name to string");
            continue;
        };

        if file.path().is_dir() {
            print!("[{}] ", file_name);
        } else {
            print!("{} ", file_name);
        }

        i = i + 1;
        if i % 6 == 0 {
            print!("\n");
        }
    }
    print!("\n");
    Ok(())
}
