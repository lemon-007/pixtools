use std::process::exit;

use image::{DynamicImage, ImageReader};
use crate::errors::LogErr;
use crate::parsing::check_path;



pub fn open_path(path: &String) -> DynamicImage{
    let valid_path = match check_path(path) {
        true => path.to_owned(),
        false => { println!("ERROR: Path not found."); exit(404) }
    };

    let image_r = ImageReader::open(valid_path)
        .log_err("Unable to get img buffreader.", 1)
        .with_guessed_format()
        .log_err("Unable to get file format.", 2);

    let decoded_img = image_r.decode().log_err("Unable to decode image at PATH", 404);
    println!("Image opened and decoded");

    decoded_img
}