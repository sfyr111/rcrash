// ch08_02_unrecoverable.rs
// 8.2 Unrecoverable Errors in Rust
// Demonstrates panic! and common patterns for unrecoverable errors.

fn main() {
    // Triggering a panic with a custom message
    // Uncomment the next line to see the panic in action
    // panic!("This is an unrecoverable error!");

    // Indexing out of bounds will cause a panic
    let v = vec![1, 2, 3];
    // let out_of_bounds = v[99]; // Uncomment to see panic

    // Assertion failure
    // assert!(1 == 2, "Assertion failed: 1 != 2");

    // Not yet implemented
    // unimplemented!("This part is not implemented yet");

    // Unreachable code
    // unreachable!("This code should never be reached");

    println!("Program completed without panic.");

    // Example: panic in a function
    // let result = always_fails();
}

// Function that always panics
fn always_fails() {
    panic!("This function always panics!");
}

// To see the full panic stack trace, set the environment variable:
// RUST_BACKTRACE=1
// Example (in terminal):
// $ RUST_BACKTRACE=1 cargo run --bin ch08_02_unrecoverable
