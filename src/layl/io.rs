use super::Result;
use std::{
    env,
    fs::{
        copy, create_dir, create_dir_all, metadata, read_dir, remove_dir, remove_dir_all,
        remove_file, File,
    },
    io::{BufRead, BufReader},
    path::Path,
};

pub fn touch(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: fcreate [filename]");
    }

    File::create(data[1])?;
    println!("touch: created file {}", data[1]);

    Ok(())
}

// What is the purpose of this, wouldn't it be better to just use echo with > >> and so on?
/*
pub fn fwrite(data: Vec<&str>) {
    if data.len() <= 2 {
        println!("usage: fwrite [filename] [content]");
        return;
    }
    let mut file = match File::create(data[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    file.write_all(data[2..].join(" ").as_bytes())
        .expect("write failed");
    println!("Wrote File");
}
*/

pub fn layl_rm(data: Vec<&str>) -> Result<()> {
    if data.len() <= 1 {
        println!("usage: rm / del [-r] [filename/foldername]");
        return Ok(());
    }

    let mut path = data[1];
    let mut recursive = false;

    if data.len() == 3 && ["-r", "--recursive"].contains(&data[1]) {
        path = data[2];
        recursive = true;
    }

    let fs_type = match metadata(path)?.is_dir() {
        true => "directory",
        false => "file",
    };

    match fs_type {
        "directory" => {
            if !recursive {
                remove_dir(path)?;
            } else {
                println!("{}: deleting {} {}", data[0], fs_type, data[1]);
                if remove_dir_all(path).is_err() {
                    println!("{}: failed to delete {} {}", data[0], fs_type, data[1]);
                    return Ok(());
                }
            }
        }
        "file" => {
            remove_file(path)?;
        }
        _ => {
            println!("{}: failed to delete {} {}", data[0], fs_type, data[1]);
            return Ok(());
        }
    }

    println!("{}: successfully deleted {} {}", data[0], fs_type, path);
    Ok(())
}

pub fn layl_copy(data: Vec<&str>) -> Result<()> {
    if data.len() <= 2 {
        println!("usage: copy / cpy [src] [dest]");
        return Ok(());
    }

    let fs_type = match metadata(data[1])?.is_dir() {
        true => "directory",
        false => "file",
    };

    match fs_type {
        "directory" => {
            create_dir_all(data[2])?;
            copy(data[1], data[2])?;
        }
        "file" => {
            copy(data[1], data[2])?;
        }
        _ => {
            println!("{}: failed to copy {} {}", data[0], fs_type, data[1]);
            return Ok(());
        }
    }

    println!("{}: copied {} {} to {}", data[0], fs_type, data[1], data[2]);

    Ok(())
}

pub fn cat(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: cat [filename]");
        return Ok(());
    }

    let file = File::open(data[1])?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(_) => {
                println!("");
            }
        }
    }
    Ok(())
}
pub fn mkdir(data: Vec<&str>) -> Result<()> {
    if data.len() != 2 {
        println!("usage: mkdir [dirname]");
        return Ok(());
    }
    create_dir(data[1])?;
    println!("mkdir: created directory {}", data[1]);
    Ok(())
}

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

    let terminal_width = term_size::dimensions_stdout().get_or_insert((80, 24)).0;

    let str: &str = if data.len() <= 1 { &path_str } else { data[1] };

    let mut i: u32 = 0;
    let fs = read_dir(str)?;
    for file in fs {
        let file = file?;

        let file_name = if let Ok(string) = file.file_name().into_string() {
            string
        } else {
            eprintln!("{}: failed to convert filename to string", data[0]);
            continue;
        };

        if file.path().is_dir() {
            i = i + file_name.len() as u32 + 3;
            print!("[{}] ", file_name);
        } else {
            i = i + file_name.len() as u32 + 1;
            print!("{} ", file_name);
        }

        //println!("term width: {}", terminal_width);
        if i % terminal_width as u32 == 0 {
            print!("\n");
        }
    }
    print!("\n");
    Ok(())
}
