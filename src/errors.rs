use std::{fmt::Debug, process::exit};

#[derive(Debug)]
pub enum ParsingError {
    MissingArguments,
    InvalidArgument,
}

pub trait LogErr<T> {
    fn log_err(self, msg: &str, code: i32) -> T;
}
impl<T, E: Debug> LogErr<T> for Result<T, E> {
    fn log_err(self, msg: &str, code: i32) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                println!("ERROR: {}", msg);
                exit(code);
            }
        }
    }
} 