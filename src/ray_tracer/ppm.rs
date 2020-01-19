use std::io::{self, Write};

pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

pub fn write_header(width: u16, height: u16) {
    println!("P6 {} {} {}", width, height, 255);
}

pub fn write_color(color: &Color) -> io::Result<()> {
    let values = vec![color.r, color.g, color.b];
    // print!("{}", &values);
    io::stdout().write(&values)?;
    Ok(())
}
