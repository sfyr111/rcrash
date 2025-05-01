# 8.2 Unrecoverable Errors in Rust

This chapter demonstrates how Rust handles unrecoverable errors using the `panic!` macro and related patterns.

## Example Code

```rust
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
```

## Key Points

- Use `panic!` to explicitly trigger an unrecoverable error.
- Use `assert!` and `assert_eq!` for runtime assertions; failures cause panic.
- Use `unimplemented!` for code that is not yet implemented.
- Use `unreachable!` for code paths that should never be executed.
- Common causes: manual panic, out-of-bounds indexing, assertion failures.
- When a panic occurs, the current thread unwinds and cleans up the stack by default.
- To see the full stack trace, set the environment variable `RUST_BACKTRACE=1` before running.
- Unrecoverable errors are meant for bugs and situations where the program cannot continue safely.
- For recoverable errors, use the `Result` type (see next section).

## Table: Unrecoverable Error Examples

| Scenario                    | Example Code                        |
|-----------------------------|-------------------------------------|
| Manual panic                | `panic!("error message")`           |
| Out-of-bounds vector access | `let x = v[99];`                    |
| Assertion failure           | `assert!(1 == 2);`, `assert_eq!(a, b);`        |
| Not implemented             | `unimplemented!("not done")`                   |
| Unreachable code            | `unreachable!("should not happen")`             |

---

Run the code:

```sh
cargo run --bin ch08_02_unrecoverable
