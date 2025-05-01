# 8.3 Recoverable Errors in Rust

This chapter demonstrates how to handle recoverable errors using the `Result` type, error propagation, and common idioms in Rust.

## Example Code

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Example 1: Opening a file (may fail)
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

    // Example 2: Propagating errors with Result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // Example 3: Using unwrap and expect (not recommended in production)
    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Function that propagates errors using Result
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

## 🏆 实战：通用文件读取与错误提示

```rust
use std::fs::File;
use std::io::{self, Read};

/// Reads the content of a file and propagates errors using Result.
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
}
```

- `read_file_content` uses the `?` operator to propagate errors, making the code concise and idiomatic.
- `main` separates business logic from error handling, and uses `eprintln!` for error output.

## Key Points

- Use the `Result<T, E>` type for operations that may fail.
- Use pattern matching (`match`) to handle success (`Ok`) and error (`Err`) cases.
- The `?` operator propagates errors, returning early if an error occurs.
- Use `unwrap()` and `expect()` only for quick prototyping or when you are sure the operation will not fail.
- Implement custom error handling logic as needed.

## Table: Recoverable Error Idioms

| Idiom                | Example Code                                  | Description                                |
|----------------------|-----------------------------------------------|--------------------------------------------|
| Pattern matching     | `match File::open("file") { ... }`           | Handle Ok/Err cases explicitly             |
| Propagation with `?` | `let f = File::open("file")?;`               | Return error to caller if any              |
| Unwrap/Expect        | `file.read_to_string(&mut s).unwrap()`        | Panic if error (not recommended in prod)   |

---

Run the code:

```sh
cargo run --bin ch08_03_recoverable
