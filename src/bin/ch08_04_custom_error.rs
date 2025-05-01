// ch08_04_custom_error.rs
// 8.4 Custom Errors and the ? Operator in Rust
// Demonstrates defining custom error types and using the ? operator for error propagation.

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// Define a custom error type that can wrap multiple error kinds
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

// Implement From for automatic conversion
impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

// A function that returns our custom error type, using the ? operator
fn read_and_parse_number(path: &str) -> Result<i32, MyError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_and_parse_number("number.txt") {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => eprintln!("Error: {}", e),
    }
}
