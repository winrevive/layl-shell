use crate::freader;
use std::fs::OpenOptions;
use crate::lyltools;




pub fn read_init(){
    let file = OpenOptions::new().read(true).open("C:\\programdata\\init.lyl");

    match file {
        Ok(_) => {
            freader::read_file("C:\\programdata\\init.lyl")
        }
        Err(_)=>{
            println!("Layl Shell\nCompiled On {}\nThis program comes with absolutely NO warranty, Use at your own risk.\n\nCopyright the Winrevive Community\n", lyltools::whats_the_version());
            return;
        }
    }
}