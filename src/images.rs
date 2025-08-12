use std::fs::File;
use std::io::{Error, ErrorKind, BufReader, Read};
use std::path::Path;
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
    let image_format = determine_format(&valid_path)
        .log_err("Invalid filetype. Refer to the list in \"pixtools -h\"");
    
    let mut image_r: ImageReader<BufReader<File>> = ImageReader::open(valid_path)
        .log_err("Unable to get img buffreader.");

    // Image passed varifications, start file manipulation.
    image_r.no_limits();
    image_r.set_format(image_format);
    
    match image_r.decode() {
        Ok(d) => { write_clear("image decoded"); return d },
        Err(e) => { write_clear("ERROR: "); print!("\r({}).\n", e); exit(1); }
    };
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
        write_clear("image found as PNG");
        //rename_img(http_path, "png");
        return Ok(ImageFormat::Png);
    } 
    
    else if buf_vect[0..3] == [255, 216, 255] {
        write_clear("image found as JPG/JPEG");
        //rename_img(http_path, "jpg");
        return Ok(ImageFormat::Jpeg);
    } 
    
    else if (buf_vect[0..4] == [82, 73, 70, 70]) && 
            (buf_vect[6..8] == [0, 0]) {
        write_clear("image found as WebP");
        //rename_img(http_path, "webp");
        return Ok(ImageFormat::WebP);
    }

    else if buf_vect[0..6] == [71, 73, 70, 56, 57, 97] {
        write_clear("image found as GIF");
        //rename_img(http_path, "gif");
        return Ok(ImageFormat::Gif)
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