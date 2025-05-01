# 7.2 Ownership in Rust

This chapter demonstrates the core concepts of ownership in Rust, including move semantics, cloning, and ownership transfer in function calls.

## Example: Ownership Basics

```rust
fn main() {
    // Ownership transfer (move)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("s1: {}", s1); // Error: value borrowed after move
    println!("s2: {}", s2);

    // Scope and ownership
    {
        let scoped = String::from("scoped");
        println!("scoped in inner scope: {}", scoped);
    } // scoped is dropped here
    // println!("scoped: {}", scoped); // Error: not found in this scope

    // Clone to copy heap data
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // Ownership and function calls (move)
    let s5 = String::from("Rust");
    takes_ownership(s5); // s5 is moved
    // println!("s5: {}", s5); // Error: value borrowed after move

    // Ownership and function calls (reference)
    let s6 = String::from("reference");
    borrows_ownership(&s6); // s6 is borrowed, not moved
    println!("s6 after borrow: {}", s6); // s6 is still valid

    let x = 5;
    makes_copy(x); // i32 is Copy, so x is still valid
    println!("x: {}", x);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn borrows_ownership(s: &String) {
    println!("borrows_ownership: {}", s);
}

fn makes_copy(x: i32) {
    println!("makes_copy: {}", x);
}
```

## Key Points

- Each value in Rust has a single owner.
- Assignment or passing ownership moves the value; the original variable becomes invalid.
- Types like `String` are moved, while simple types like `i32` are `Copy` and remain valid after assignment.
- Use `.clone()` to explicitly duplicate heap data.
- Ownership is transferred when passing values to functions, unless the type implements `Copy`.
- You can borrow values using references (`&T`), allowing functions to access data without taking ownership.
- Values are dropped (memory freed) when they go out of scope.

## Table: Ownership Behavior

| Type     | Assignment | Function Call | Clone   |
|----------|------------|---------------|---------|
| String   | Move       | Move          | Deep copy |
| i32      | Copy       | Copy          | Copy    |

## Notes

- Ownership is the foundation of Rust's memory safety.
- Attempting to use a moved value results in a compile-time error.
