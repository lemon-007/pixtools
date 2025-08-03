mod errors;
mod parsing;
mod http;
mod images;

use std::{env, process::exit};
use errors::ParsingError;
use parsing::METHOD;

use crate::parsing::{sort_tokens, tokenize_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("No arguments. type \"pixtools help\"."); exit(1)}

    // A URL should always be the 2nd argument.
    let file_location: String =  parse_url(&args).unwrap_or_else(|error| {
        println!("ERROR: This is not a valid URL. Dickhead. Make sure it's an image too. also type \"pixtools help\". ({:?})", error);
        exit(1);
    });

    // Get each of the arguments, file them into order of operation. This is needed so we don't do things wrong.
    let tokens: Vec<parsing::TOKEN> = tokenize_args(&args);
    let order: Vec<parsing::TOKEN> = sort_tokens(&tokens);
    println!("Success! Here is your url: \"{}\" and are your tokens in order: {:?}", file_location, order); // turn off later

    let _img = match order.contains(&parsing::TOKEN::PATH) {
        true => http::get_img(&file_location, METHOD::PATH),
        false => http::get_img(&file_location, METHOD::URL),
    };

    order.into_iter().for_each(|_t| {
        todo!() // Put it through each function that could be a token. Im done for now.
    });
}

fn parse_url(args: &Vec<String>) -> Result<String, ParsingError> {
    if args.len() <= 1 {
        return Err(ParsingError::MissingArguments);
    }

    if args[1].contains("help") {
        display_help_message();
        exit(0);
    } 
    
    else if !args[1].contains("url") {
        return Err(ParsingError::InvalidArgument);
    }

    let arg: &String = &args[1];
    Ok(arg.to_string())
}

fn display_help_message() {
    println!("\nPixtools cleans pictures with from weird URLs and saves it to your downloads.");
    println!("Unfortunately saving them from Discord is not enough. Here's how you do it:\n");
    println!("pixtools *URL/PATH* *TOKENS*");
    println!("pixtools https://www.mdxblog.io/images/posts/how-to-use-images/grass-tree-sky.jpg -cl -gif\n");
    println!("Pixtools commands:
    -clean -cl  Removes all embeds from an image, results in a new clean file saved to your downloads.
    -path       Forces Pixtools to use a file path instead of a url (the default) This command works with -clean.
    -gif        Converts the image to .gif before saving. This command is works with -clean and -path.
    -png        Converts the image to .png before saving. This command is works with -clean and -path.
    help        Displays this message. Only if it's the only command though.
    ");
    println!("Pixtools only does what you tell it to. If you add no tokens it simply downloads the image from a url.");
}