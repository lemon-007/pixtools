use std::{fmt::Debug, io::{stdout, Write}, process::exit};

#[derive(Debug)]
pub enum ParsingError {
    MissingArguments,
    _InvalidArgument,
}

pub trait LogErr<T> {
    fn log_err(self, msg: &str) -> T;
}
impl<T, E: Debug> LogErr<T> for Result<T, E> {
    fn log_err(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                let clear_str = format!("\r{}\r", " ".repeat(80));
                print!("{clear_str}\rERROR: ");
                print!("{}.\n", msg);
                stdout().flush().unwrap();
                exit(1);
            }
        }
    }
}