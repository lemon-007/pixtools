use std::fs::File;
use std::io::{Error, ErrorKind, BufReader, Read};
use std::path::Path;
use std::process::exit;

use image::ImageFormat;
use crate::errors::{LogErr};
use crate::parsing::{check_path};



pub fn open_path(path: &String) -> BufReader<File>{
    let valid_path = match check_path(path) {
        true => path.to_owned(),
        false => { println!("ERROR: Invalid path\n"); exit(404) }
    };

    determine_format(&valid_path).log_err("Invalid filetype. Refer to the list in \"pixtools -h\"");
    let file = File::open(Path::new(&valid_path))
        .log_err("Unable to open file from drive");

    let image_reader: BufReader<File> = BufReader::new(file);
    image_reader
}

// Gets the file signiture of the current file to determine image format
fn determine_format(p: &String) -> Result<ImageFormat, Error> {
    let file = File::open(Path::new(p)).log_err("Unable to open file (determine_format)");
    let mut reader = BufReader::new(&file);

    // "first_8" represents the first 8 bytes of a file. This is where the signature is.
    let mut first_8: [u8; 8] = [0; 8];
    reader.read_exact(&mut first_8).log_err("Unable to get file signature.");
    let buf_vect: Vec<u8> = first_8.to_vec(); // Converted to managable vector
    drop(file);

    let _http_path: bool = match p.as_str() {
        "temp/response.unknown" => true,
        _ => false
    };

    // Parsing file signatures
    if buf_vect == [137, 80, 78, 71, 13, 10, 26, 10] {
        println!("image found as PNG");
        //rename_img(http_path, "png");
        return Ok(ImageFormat::Png);
    } 

    else {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid filetype."));
    }
}

fn _rename_img(if_: bool, s: &str) {
    if if_ {
        let pt1 = "response";
        let new_name = format!("temp/{}.{}", pt1, s);
        println!("{:?}", new_name);
        std::fs::rename("temp/response.unknown", new_name).log_err("Unable to rename file from .unknown to .<ext>")
    }
}       