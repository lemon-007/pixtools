use std::process::exit;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum TOKEN {
    CLEAN,
    PATH,
    GIF,
    PNG,
    ERR { wrong_token: String }
}

// Turns shitty strings into clearly computable tokens
pub fn tokenize_args(args: &Vec<String>) -> Vec<TOKEN> {
    let mut tokens: Vec<TOKEN> = args.iter().map(|arg| {
        if matches!(arg.as_str(), "-clean" | "cl") { TOKEN::CLEAN }
        else if matches!(arg.as_str(), "-path" | "-p") { TOKEN::PATH }
        else if matches!(arg.as_str(), "-gif") { TOKEN::GIF }
        else if matches!(arg.as_str(), "-png") { TOKEN::PNG }
        else { TOKEN::ERR{ wrong_token: arg.to_owned() } }
    }).collect();

    tokens.drain(0..2);
    return tokens
}

// Order: Path Type (path or url), Mode (clean or not), Extention (png, url, etc).
pub fn sort_tokens(tokens: &Vec<TOKEN>) -> Vec<TOKEN> {
    let mut sorted_tokens: Vec<TOKEN> = Vec::new();
    let mut fucked: bool = false;

    if tokens.contains(&TOKEN::PATH) { sorted_tokens.push(TOKEN::PATH); }
    if tokens.contains(&TOKEN::CLEAN) { sorted_tokens.push(TOKEN::CLEAN); }
    if tokens.contains(&TOKEN::GIF) { sorted_tokens.push(TOKEN::GIF); }
    if tokens.contains(&TOKEN::PNG) { sorted_tokens.push(TOKEN::PNG); }

    if tokens.contains(&TOKEN::GIF) && tokens.contains(&TOKEN::PNG) {
        println!("ERROR: You can't convert your image into 2 different types. (InvalidArgument)");
        fucked = true;
    }

    for t in tokens {
        if let TOKEN::ERR { wrong_token } = t {
            println!("ERROR: Unreadable token ({}).", wrong_token);
            fucked = true;
        }
    }
    if fucked { 
        println!("You must be retarded. That's ok. Type \"pixtools help\" for help.");
        exit(1) 
    }

    if sorted_tokens.len() < 1 { println!("Did you really just give me a NULL vector? Fuck you. Add some more args next time.") }
    return sorted_tokens
}

pub fn check_path(path: &String) -> bool {
    let path_eval = Path::new(path);
    path_eval.exists()
}