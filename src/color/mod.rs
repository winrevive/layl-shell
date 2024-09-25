use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};





pub fn color_chooser(data: Vec<&str>){
    if data.len() <= 3 {
        println!("usage: color [r] [g] [b]");
        return;
    }

    let r = match data[1].to_lowercase().parse::<u8>() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            0
        }
    };
    let g = match data[2].to_lowercase().parse::<u8>() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            0
        }
    };
    let b = match data[3].to_lowercase().parse::<u8>() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            0
        }
    };

    match write_color(Color::Rgb(r, g, b)) {
        Ok(_) => {
            println!("Color Changed");
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
    
    
}



fn write_color(color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "")
}