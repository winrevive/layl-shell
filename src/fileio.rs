use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};

pub fn touch(data: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if data.len() <= 1 {
        println!("usage: touch [filename]");
    }
    let file = File::create(data[1])?;
    Ok(())
}

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

pub fn fdelete(data: Vec<&str>) {
    if data.len() <= 1 {
        println!("usage: fdelete [filename]");
        return;
    }
    match fs::remove_file(data[1]) {
        Ok(_) => {
            println!("Deleted File\n");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn fcopy(data: Vec<&str>) {
    if data.len() <= 2 {
        println!("usage: fcopy [originalfile] [newfile]");
        return;
    }
    match fs::copy(data[1], data[2]) {
        Ok(_) => {
            println!("Copied File\n");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

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

pub fn fread(data: Vec<&str>) {
    if data.len() <= 1 {
        println!("usage: fread [filename]");
        return;
    }
    let file = File::open(data[1]);
    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);
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
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

