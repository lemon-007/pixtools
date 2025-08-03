use image::DynamicImage;

use crate::parsing::METHOD;
use crate::images::open_path;

// Test link: https://www.soyjak.st/soy/src/1754175526539v-0.png
pub fn get_img(path: &String, pathtype: METHOD) -> DynamicImage {
    if pathtype == METHOD::PATH {
        return open_path(path);
    } else {
        todo!()
    }
}