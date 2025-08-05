use reqwest::blocking;
use image::DynamicImage;

// Test link: https://www.soyjak.st/soy/src/1754175526539v-0.png
pub fn open_url(url: &String) -> DynamicImage {
    let res_img = blocking::get(url).unwrap();
    println!("{:?}", res_img);
    todo!()
}