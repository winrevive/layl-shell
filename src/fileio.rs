use super::Error;
use std::fs::{self, File};
use std::io::{self, BufRead};

pub fn touch(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: touch [filename]");
        return Ok(());
    }
    File::create(data[1])?;
    Ok(())
}

// I don't know what this is for, isn't this shell supposed to support pipes or writing output into files?
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

pub fn fdel(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: fdel [path\\to\\filename]");
        return Ok(());
    }
    fs::remove_file(data[1])?;
    println!("Deleted file: {}", data[1]);
    Ok(())
}

pub fn fcopy(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 2 {
        println!("usage: fcopy [path\\of\\original\\file] [path\\of\\newfile]");
        return Ok(());
    }
    fs::copy(data[1], data[2])?;
    println!("Copied file: {} to {}", data[1], data[2]);
    Ok(())
}

// I don't know what this is for, isn't this shell supposed to support pipes or writing output into files?
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

pub fn cat(data: Vec<&str>) -> Result<(), Error> {
    if data.len() <= 1 {
        println!("usage: cat [filename]");
        return Ok(());
    }
    let file = File::open(data[1])?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
