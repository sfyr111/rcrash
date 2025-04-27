# Chapter 4.15: Guessing Game

## Key Points

- Demonstrates standard input/output, random number generation, and control flow in Rust.
- Uses `rand` crate for generating random numbers.
- Shows how to read user input, parse to number, and handle errors gracefully.
- Uses `match` and `Ordering` for comparison logic.

## Demo Code

```rust
use std::io;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..=100);
    // Uncomment the next line to debug:
    // println!("(Debug) The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Table: Guessing Game Flow

| Step                | Description                                   |
|---------------------|-----------------------------------------------|
| Generate secret     | Random number between 1 and 100               |
| Read input          | User enters guess as a string                 |
| Parse input         | Convert string to `u32`, handle parse errors  |
| Compare             | Use `cmp` to compare guess and secret         |
| Give feedback       | Print "Too small", "Too big", or "You win!"   |

## Notes

- Make sure to add `rand` to your `Cargo.toml` dependencies:
  ```toml
  rand = "0.8"
  ```
- This is a classic Rust beginner project, covering basic syntax and error handling.
- You can uncomment the debug line to see the secret number for testing.

---

> See also: [Rust Book: Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
