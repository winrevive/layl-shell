use super::{cmdchecker, Result};
use std::{
    fs::{
        copy,
        remove_file,
        // OpenOptions
        File,
    },
    io::{
        BufRead,
        BufReader,
        // Write
    },
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

pub fn fdel(data: Vec<&str>) -> Result<()> {
    if data.len() <= 1 {
        println!("usage: fdel [filename]");
        return Ok(());
    }

    remove_file(data[1])?;
    println!("fdel: deleted file {}\n", data[1]);

    Ok(())
}

pub fn fcopy(data: Vec<&str>) -> Result<()> {
    if data.len() <= 2 {
        println!("usage: fcopy [originalfile] [destfile]");
        return Ok(());
    }

    copy(data[1], data[2])?;
    println!("fcopy: copied file {} to {}\n", data[1], data[2]);

    Ok(())
}
// What is the purpose of this, wouldn't it be better to just use echo with > >> and so on?
/*
pub fn fprint(data: Vec<&str>) {
    if data.len() <= 2 {
        println!("usage: fprint [filename] [content]");
        return;
    }
    let mut file = match OpenOptions::new().append(true).write(true).open(data[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let status = writeln!(file, "{}\n", data[2..].join(" "));
    match status {
        Ok(_) => {
            println!("Wrote File!");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
*/

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

pub fn read_file(filename: &str) -> Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        cmdchecker::cmd_checker(line.split_whitespace().collect(), &line)?;
    }
    Ok(())
}
