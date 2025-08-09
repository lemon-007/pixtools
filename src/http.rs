use std::process::exit;
use std::fs::{self};

use tokio::fs::File;
use image::DynamicImage;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::errors::LogErr;
use crate::parsing::write_clear;
use crate::images::open_path;

// Test link: https://culturesnapshots.com/wp-content/uploads/2022/06/ulleungdo-featured.jpg
// Test PATH: /home/nate/projects/pixtools/src/testimg/superchudpng.png 
pub async fn open_url(url: &String) -> DynamicImage {
    print!("opening url");

    // Get request data
    let client = Client::builder()
        .user_agent("PixtoolsClient/1.0")
        .build()
        .unwrap();

    write_clear("sending GET request to url");
    let mut res_img = client.get(url).send().await
        .unwrap_or_else(|e| {
            write_clear("ERROR: Unable to send request");
            print!(" ({}).\n", e);
            exit(1);
        });

    // Get name for new file that stores request data
    if !res_img.status().is_success() {
        write_clear("ERROR: Request to get image failed");
        print!(" ({}).\n", res_img.status());
        exit(1);
    }

    write_clear("creating new temp files");
    fs::create_dir("temp").log_err("Unable to create new TEMP directory for new file");
    let mut file = File::create_new("temp/response.img").await.unwrap_or_else(|err| {
        write_clear("ERROR: Unable to create temp file");
        print!("{}", err);
        exit(1) ;
    });

    write_clear("writing to temp file");
    while let Some(chunk) = res_img.chunk().await.log_err("Unable to chunk file.") {
        file.write_all(&chunk).await.log_err("Unable to write chunk to file.");
    }

    write_clear("opening path");
    open_path(&String::from("temp/response.img")) // Its not broken. Im not fixing it.
}