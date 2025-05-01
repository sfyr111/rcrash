# 8.4 Custom Errors and the `?` Operator in Rust

This chapter demonstrates how to define custom error types in Rust, implement error conversion, and use the `?` operator for ergonomic error propagation.

## Example Code

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

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
```

## Key Points

- Define custom error enums to unify multiple error types.
- Implement `Display` and `Error` traits for better error messages and compatibility.
- Implement `From` for automatic error type conversion, enabling the use of `?` with different error types.
- The `?` operator automatically converts errors using `From`.
- This pattern is idiomatic for robust error handling in real-world Rust applications.

---

Run the code:

```sh
cargo run --bin ch08_04_custom_error
```
