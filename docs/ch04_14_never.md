# Chapter 4.14: Diverging Functions (Never Type)

## Key Points

- Diverging functions are functions that never return to the caller.
- Their return type is `!` (the never type).
- Common examples: `panic!()`, infinite loops (`loop {}`), or calling other diverging functions.
- Useful for error handling, stubs, or cases where code execution should not continue.

## Demo Code

```rust
// Demonstration of diverging (never type) functions in Rust

// A diverging function never returns, its return type is `!` (never type)
fn never_returns() -> ! {
    panic!("This function never returns!");
}

fn infinite_loop() -> ! {
    loop {
        // Infinite loop, never returns
    }
}

fn main() {
    println!("About to call a diverging function...");
    // Uncomment one of the lines below to see diverging behavior:
    // never_returns();
    // infinite_loop();
    println!("This line is after diverging functions (unreachable if called)");
}
```

## Table: Diverging Function Patterns

| Pattern                | Example                   | Description                             |
|------------------------|---------------------------|-----------------------------------------|
| Panic                  | `panic!("msg")`           | Aborts execution, never returns         |
| Infinite loop          | `loop {}`                 | Runs forever, never returns             |
| Function returning `!` | `fn foo() -> ! { ... }`   | Explicit diverging function             |

## Notes

- Diverging functions can be used in places where a value is expected, thanks to the never type `!` being coercible into any other type.
- Useful for implementing stubs or handling unrecoverable errors.
- Rust's type system uses `!` to ensure code after diverging calls is unreachable.

---

> See also: [Rust Book: Never Type](https://doc.rust-lang.org/nomicon/never-type.html)
