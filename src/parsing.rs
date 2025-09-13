use std::io::{stdout, Write};
use std::process::exit;
use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum TOKEN {
    CLEAN,
    PATH,
    GIF,
    PNG,
    HELP,
    FILLOC { str: String}, // file location
    //ERR { wrong_token: String }
}

// Turns strings into tokens, errors will be thrown out during path/url check.
pub fn tokenize_args(args: &Vec<String>) -> Vec<TOKEN> {
    let mut tokens: Vec<TOKEN> = args.iter().map(|arg| {
        if matches!(arg.as_str(), "-clean" | "cl") { TOKEN::CLEAN }
        else if matches!(arg.as_str(), "-path" | "-p") { TOKEN::PATH }
        else if matches!(arg.as_str(), "-gif") { TOKEN::GIF }
        else if matches!(arg.as_str(), "-png") { TOKEN::PNG }
        else if matches!(arg.as_str(), "-help" | "-h") { TOKEN::HELP }
        else { TOKEN::FILLOC { str: arg.to_string() } }
    }).collect();

    // First argument collected is always "pixtools"
    tokens.drain(0..1);
    return tokens
}

// Order: Path Type (path or url), Mode (clean or not), Extention (png, url, etc).
pub fn sort_tokens(tokens: &Vec<TOKEN>) -> Vec<TOKEN> {
    let mut sorted_tokens: Vec<TOKEN> = Vec::new();
    let mut invalid_token: bool = false;

    for t in tokens {
        let clone = t.to_owned().clone();
        if let TOKEN::FILLOC { str: _ } = t {
            sorted_tokens.push(clone);
        }
    }

    if tokens.contains(&TOKEN::PATH) { sorted_tokens.push(TOKEN::PATH); }
    if tokens.contains(&TOKEN::CLEAN) { sorted_tokens.push(TOKEN::CLEAN); }
    if tokens.contains(&TOKEN::GIF) { sorted_tokens.push(TOKEN::GIF); }
    if tokens.contains(&TOKEN::PNG) { sorted_tokens.push(TOKEN::PNG); }

    if tokens.contains(&TOKEN::GIF) && tokens.contains(&TOKEN::PNG) {
        println!("ERROR: You can't convert your image into 2 different types. (InvalidArgument)");
        invalid_token = true;
    }

    // for t in tokens {
    //     if let TOKEN::ERR { wrong_token } = t {
    //         println!("ERROR: Unreadable token ({}).", wrong_token);
    //         invalid_token = true;
    //     }
    // }
    // if invalid_token { 
    //     println!("ERROR: Invalid token: Type \"pixtools help\" for help.");
    //     exit(1) 
    // }
    return sorted_tokens
}

pub fn check_path(path: &String) -> bool {
    let path_eval = Path::new(path);
    path_eval.exists()
}

pub fn _input() -> String {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    str
}