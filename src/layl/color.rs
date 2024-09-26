use super::Result;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn set_fg_color(color: Color) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "")?;
    Ok(())
}

fn set_bg_color(color: Color) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bg(Some(color)))?;
    writeln!(&mut stdout, "")?;
    Ok(())
}

enum ColorType {
    Rgb,
    Hex,
}

pub fn color_chooser(data: Vec<&str>) -> Result<()> {
    if data.len() != 3 {
        println!("usage: color [fg/bg] [rrrgggbbb/#hex/clear]");
        return Ok(());
    }

    let color_type = data[1];
    let mut color_code = data[2];
    let color: Color;

    let r: u8;
    let g: u8;
    let b: u8;

    if color_code == "clear" {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.reset()?;
        writeln!(&mut stdout, "")?;
        return Ok(());
    }

    let mut len = color_code.len();
    let mut v = vec![];
    let type_of_color: ColorType;

    if color_code.starts_with("#") {
        type_of_color = ColorType::Hex;
        color_code = color_code.trim_start_matches("#");
        len -= 1;
    } else {
        type_of_color = ColorType::Rgb;
    }

    while !color_code.is_empty() {
        let (chunk, rest) = color_code.split_at(std::cmp::min(len / 3, color_code.len()));
        v.push(chunk);
        color_code = rest;
    }

    let (first, middle, last) = (v[0], v[1], v[2]);

    match type_of_color {
        ColorType::Rgb => {
            r = first.parse()?;
            g = middle.parse()?;
            b = last.parse()?;
        }
        ColorType::Hex => {
            r = u8::from_str_radix(first, 16)?;
            g = u8::from_str_radix(middle, 16)?;
            b = u8::from_str_radix(last, 16)?;
        }
    }

    color = Color::Rgb(r, g, b);

    match color_type {
        "fg" => {
            set_fg_color(color)?;
        }
        "bg" => {
            set_bg_color(color)?;
        }
        _ => {
            println!("usage: color [fg/bg] [rrrgggbbb/#hex/clear]");
        }
    }
    Ok(())
}
