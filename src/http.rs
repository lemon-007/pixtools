use std::path::PathBuf;
use std::io::BufReader;
use std::process::exit;
use std::fs::{self, File};

use reqwest::{Client, StatusCode};
use tokio::io::AsyncWriteExt;
use tokio::fs::File as TokFile;

use crate::errors::LogErr;
use crate::images::open_path;

// Test links: 
// PNG: https://w.wallhaven.cc/full/4y/wallhaven-4yld9x.png
// PNG: https://w.wallhaven.cc/full/pk/wallhaven-pk6629.jpg
// JPG: https://w.wallhaven.cc/full/q6/wallhaven-q6go6l.jpg
pub async fn open_url(url: &String) -> BufReader<File> {
    print!("opening url");

    // Get request data
    let client = build_shared_client();
    let mut response_img = client.get(url).send().await
        .unwrap_or_else(|e| {
            println!("ERROR: Unable to send request");
            print!(" ({}).\n", e);
            exit(1);
        });

    if !response_img.status().is_success() {
        match response_img.status() {
            StatusCode::NOT_FOUND => {
                println!("ERROR: HTTP code 404, double check your URL.");
                exit(1);
            },
            
            _ => {
                println!("ERROR: {}.", response_img.status().as_str());
                exit(1);
            }
        }
    }

    let _ = fs::remove_dir_all("temp");
    fs::create_dir("temp").log_err("Unable to create new TEMP directory for new file");

    let mut new_path = PathBuf::from("temp");
    new_path.push("response.unknown");
    let mut file = TokFile::create_new(&new_path).await.log_err("Unable to create directory and new file");

    while let Some(chunk) = response_img.chunk().await.log_err("Unable to chunk file.") {
        file.write_all(&chunk).await.log_err("Unable to write chunk to file.");
    }

    open_path(&new_path.into_os_string()
        .into_string()
        .log_err("Unable to turn OS path into string"))
}

// Will probably have to use this for other things in the future
pub fn build_shared_client() -> reqwest::Client {
    let client = Client::builder()
        .user_agent("PixtoolsClient/1.0")
        .build()
        .unwrap();

    client
}