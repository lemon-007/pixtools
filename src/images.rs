use std::fs::File;
use std::io::{Error, ErrorKind, BufReader};
use std::process::exit;

use image::{DynamicImage, ImageFormat, ImageReader};
use crate::errors::{LogErr};
use crate::parsing::{check_path, write_clear};



pub fn open_path(path: &String) -> DynamicImage{
    write_clear("checking if PATH is valid");
    let valid_path = match check_path(path) {
        true => path.to_owned(),
        false => { write_clear("ERROR: Invalid path\n"); exit(404) }
    };

    write_clear("checking if file type is valid");
    let mut image_r: ImageReader<BufReader<File>> = ImageReader::open(valid_path)
        .log_err("Unable to get img buffreader.");
    determine_format(&image_r).log_err("Invalid filetype. Refer to the list in \"pixtools -h\"");

    // Image passed varifications, start file manipulation.
    image_r.no_limits();
    match image_r.decode() {
        Ok(d) => { write_clear("image decoded"); return d },
        Err(e) => { write_clear("ERROR: "); print!("\r({}).\n", e); exit(1); }
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
            write_clear("No image format found. Odd. Its almost like you should use an image.\n ");
            Err(Error::new(ErrorKind::NotFound, "No Image Format"))
        },
    }
}