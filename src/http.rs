use std::path::PathBuf;
use std::process::exit;
use std::fs::{self};

use tokio::fs::File;
use image::DynamicImage;
use reqwest::{Client, Response};
use tokio::io::AsyncWriteExt;

use crate::errors::LogErr;
use crate::images::open_path;

// Test links: 
// PNG: https://w.wallhaven.cc/full/4y/wallhaven-4yld9x.png
// PNG: https://w.wallhaven.cc/full/pk/wallhaven-pk6629.jpg
// JPG: https://w.wallhaven.cc/full/q6/wallhaven-q6go6l.jpg
pub async fn open_url(url: &String) -> DynamicImage {
    print!("opening url");

    // Get request data
    let client = build_shared_client();
    let mut res_img = client.get(url).send().await
        .unwrap_or_else(|e| {
            println!("ERROR: Unable to send request");
            print!(" ({}).\n", e);
            exit(1);
        });

    // Get name for new file that stores request data
    if !res_img.status().is_success() {
        println!("ERROR: Request to get image failed");
        print!(" ({}).\n", res_img.status());
        exit(1);
    }

    let _ = fs::remove_dir_all("temp");
    fs::create_dir("temp").log_err("Unable to create new TEMP directory for new file");

    let mut new_path = PathBuf::from("temp");
    new_path.push("response.unknown");
    let mut file = File::create_new(&new_path).await.log_err("Unable to create directory and new file");

    while let Some(chunk) = res_img.chunk().await.log_err("Unable to chunk file.") {
        file.write_all(&chunk).await.log_err("Unable to write chunk to file.");
    }

    open_path(&new_path.into_os_string()
        .into_string()
        .log_err("Unable to turn OS path into string"))
}

pub async fn test_url(url: &String, http_client: &reqwest::Client) -> bool {
    let response = send_get(url, http_client).await;
    response.status().is_success()
}

pub async fn send_get(url: &String, http_client: &reqwest::Client) -> Response {
    let response = http_client.get(url).send().await
        .unwrap_or_else(|e| {
            println!("ERROR: Unable to send request");
            print!(" ({}).\n", e);
            exit(1);
        });
    response
}

// Shouldn't have to define a new client every single time we test a link
pub fn build_shared_client() -> reqwest::Client {
    let client = Client::builder()
        .user_agent("PixtoolsClient/1.0")
        .build()
        .unwrap();

    client
}