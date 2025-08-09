use std::process::exit;
use std::fs::{self, File};

use image::DynamicImage;
use reqwest::Client;

use crate::errors::LogErr;
use crate::parsing::input;

// Test link: https://www.soyjak.st/soy/src/1754175526539v-0.png
// Test PATH: /home/nate/projects/pixtools/src/testimg/superchudpng.png 
pub async fn open_url(url: &String) -> DynamicImage {
    
    // Get request data
    let client = Client::builder()
        .user_agent("PixtoolsClient/1.0")
        .build()
        .unwrap();

    let res_img = client.get(url).send().await
        .unwrap_or_else(|e| {
            println!("ERROR: unable to send request ({})", e);
            exit(1);
        });

    // Get name for new file that stores request data
    if !res_img.status().is_success() {
        println!("ERROR: Request to get image failed ({})", res_img.status());
        exit(1);
    }

    // New idea, only ask for new file name before final file creation.
    fs::create_dir("temp").log_err("Unable to create new TEMP directory for new file", 1);
    let mut _file = File::create("temp/response.bytes").unwrap_or_else(|err| {
        println!("ERROR: Unable to create .bytes file ({})", err);
        exit(1) ;
    });
    println!(".bytes file created");

    return DynamicImage::new(4, 4, image::ColorType::Rgba16);
}