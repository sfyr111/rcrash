// Guessing game demo in Rust
// This example demonstrates basic I/O, random number generation, and control flow.

use std::io;
use rand::{Rng, rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rng().random_range(1..=100);
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
