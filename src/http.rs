use crate::parsing::METHOD;
use crate::images::open_path;

// Test link: https://www.soyjak.st/soy/src/1754175526539v-0.png
pub fn get_img(path: &String, pathtype: METHOD) {
    if pathtype == METHOD::PATH {
        let _img = open_path(path);
    }
}