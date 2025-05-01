// ch08_03_recoverable.rs
// 8.3 Recoverable Errors in Rust
// Demonstrates the use of Result, error propagation, and handling recoverable errors.

use std::fs::File;
use std::io::{self, Read};

/// Reads the content of a file and propagates errors using Result.
/// 
/// This function is a best practice example of how to handle file I/O operations.
/// It uses the `?` operator to propagate errors up the call stack, allowing the caller to handle the error.
fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename = "hello.txt";
    match read_file_content(filename) {
        Ok(text) => println!("File content:\n{}", text),
        Err(e) => eprintln!("Failed to read file [{}]: {}", filename, e),
    }

    // Example: Using Result with match
    /// This example demonstrates how to use `match` to handle the `Result` returned by `File::open`.
    /// It's a good practice to handle errors explicitly, rather than using `unwrap` or `expect`.
    match File::open("hello.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or(0);
            println!("File opened successfully: {}", contents);
        }
        Err(e) => {
            println!("Failed to open file: {}", e);
        }
    }

    // Example: Propagating errors with Result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // Example: Using unwrap and expect (not recommended in production)
    // These methods will panic if an error occurs, which is not desirable in production code.
    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Example function that propagates errors using Result
/// This function demonstrates how to propagate errors using the `?` operator.
/// It's a good practice to use `Result` to handle errors explicitly, rather than using `unwrap` or `expect`.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
