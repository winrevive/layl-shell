use super::Error;
use std::env;
use std::fs;
use std::path::Path;

fn get_current_working_dir() -> Result<Option<String>, Error> {
    let path = env::current_dir()?;
    Ok(Some(path.display().to_string()))
}

pub fn mkdir(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: mkdir [dirname]");
        return Ok(());
    }
    fs::create_dir(data[1])?;
    println!("Created Directory: {}", data[1]);
    Ok(())
}

// pub fn dcopy(data: Vec<&str>){
//     // TODO: Add Recursive Directory Copying
// }

pub fn deldir(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: deldir [-r] [dirname]");
        return Ok(());
    }

    match data[1] {
        "-r" => {
            fs::remove_dir_all(data[2])?;
            println!("Deleted Directory: {}", data[2]);
        }
        _ => {
            fs::remove_dir(data[1])?;
            println!("Deleted Directory: {}", data[1]);
        }
    }
    Ok(())
}

pub fn change_directory(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        let path = env::current_dir()?;
        println!("{}", path.display());
        return Ok(());
    }
    let path = Path::new(data[1]);
    env::set_current_dir(path)?;
    Ok(())
}

pub fn print_directory(data: Vec<&str>) -> Result<(), Error> {
    let path_str = match get_current_working_dir()? {
        Some(p) => p,
        None => return Err("Error Getting Current Working Directory".into()),
    };

    let str: &str = if data.len() <= 1 { &path_str } else { data[1] };

    let mut i: u32 = 0;
    let fs = fs::read_dir(str);
    match fs {
        Ok(fs) => {
            for file in fs {
                match file {
                    Ok(fil) => {
                        let file_name = fil.file_name();
                        match fil.metadata() {
                            Ok(fi) => {
                                if fi.is_dir() {
                                    print!("[{:?}] ", file_name);
                                } else {
                                    print!("{:?} ", file_name);
                                }
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                        i = i + 1;
                        if i % 6 == 0 {
                            println!()
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    println!();
    Ok(())
}
