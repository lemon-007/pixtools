use std::{path::Path, process::exit};

#[derive(PartialEq, Debug, Clone)]
pub enum TOKEN {
    CLEAN,
    PATH,
    HELP,
    FILLOC { str: String}, // file location
    //ERR { wrong_token: String }
}

// Turns strings into tokens, errors will be thrown out during path/url check.
pub fn tokenize_args(args: &Vec<String>) -> Vec<TOKEN> {
    let mut tokens: Vec<TOKEN> = args.iter().map(|arg| {
        if matches!(arg.as_str(), "-clean" | "cl") { TOKEN::CLEAN }
        else if matches!(arg.as_str(), "-path" | "-p") { TOKEN::PATH }
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
    let mut misc_token: usize = 0; // There can't be more than one file path. So if there is two, something is wrong.
    if tokens.contains(&TOKEN::HELP) { super::display_help_message(); exit(0); }

    for t in tokens {
        let clone = t.to_owned().clone();
        if let TOKEN::FILLOC { str: _ } = t {
            sorted_tokens.push(clone);
            misc_token += 1;
        }
    }

    if tokens.contains(&TOKEN::PATH) { sorted_tokens.push(TOKEN::PATH); }
    if tokens.contains(&TOKEN::CLEAN) { sorted_tokens.push(TOKEN::CLEAN); }
    if misc_token > 1 { println!("ERROR: Invalid argument(s)"); }
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