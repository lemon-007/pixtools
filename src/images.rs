use std::fs::File;
use std::io::{Error, ErrorKind, BufReader};
use std::process::exit;

use image::{DynamicImage, ImageFormat, ImageReader};
use crate::errors::{LogErr};
use crate::parsing::check_path;



pub fn open_path(path: &String) -> DynamicImage{
    let valid_path = match check_path(path) {
        true => path.to_owned(),
        false => { println!("ERROR: Invalid path"); exit(404) }
    };

    let mut image_r: ImageReader<BufReader<File>> = ImageReader::open(valid_path)
        .log_err("Unable to get img buffreader.", 1);
    determine_format(&image_r).log_err("Invalid filetype. Refer to the list in \"pixtools -h\"", 599);

    // Image passed varifications, start file manipulation.
    image_r.no_limits();
    match image_r.decode() {
        Ok(d) => { println!("Image decoded at PATH ({}).", path); return d },
        Err(e) => { println!("ERROR: {}", e.to_string()); exit(1); }
    };
}

// I don't like really long code horizontally, so you will have to deal with all these statements.
fn determine_format(image_reader: &ImageReader<BufReader<File>>) -> Result<(), Error> {
    match image_reader.format() {
        Some(f) if f == ImageFormat::Png => Ok(()),
        Some(f) if f == ImageFormat::Jpeg => Ok(()),
        Some(f) if f == ImageFormat::Gif => Ok(()),
        Some(f) if f == ImageFormat::WebP => Ok(()),
        Some(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid filetype")),
        None => {
            println!("No image format found. Odd. Its almost like you should use an image.");
            Err(Error::new(ErrorKind::NotFound, "No Image Format"))
        },
    }
}