use std::fs;
use std::env;
use std::path::Path;



fn get_current_working_dir() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.into_os_string().into_string().unwrap()),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

pub fn dcreate(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: dcreate [dirname]");
        return;
    }
    let status = fs::create_dir(data[1]);
    match status {
        Ok(_) => {
            println!("Created Directory");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn ddelete(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: ddelete [dirname]");
        return;
    }
    
    let status = fs::remove_dir(data[1]);
    match status {
        Ok(_) => {
            println!("Deleted Directory");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

// pub fn dcopy(data: Vec<&str>){
//     // TODO: Add Recursive Directory Copying
// }

pub fn rddelete(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: rddelete [dirname]");
        return;
    }
    
    let status = fs::remove_dir_all(data[1]);
    match status {
        Ok(_) => {
            println!("Deleted Directory");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn change_directory(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: moveto [dirname]");
        return;
    }
    let path = Path::new(data[1]);
    let status = env::set_current_dir(path);
    match status {
        Ok(_) => {
            println!("Changed Directory To {}", path.display());
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn current_directory() {
    let path = env::current_dir();
    match path {
        Ok(path) => {
            println!("Current Directory Is {}", path.display());
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn print_directory(data: Vec<&str>) {
    let path_str = match get_current_working_dir() {
        Some(p) => p,
        None => return,
    };

    let str: &str = if data.len() <= 1 {
        &path_str
    } else {
        data[1]
    };

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
                                }
                                else {
                                    print!("{:?} ", file_name);
                                }
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                        i = i+1;
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
}   