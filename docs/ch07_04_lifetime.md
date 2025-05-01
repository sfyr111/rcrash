# 7.4 Lifetimes in Rust

This chapter demonstrates Rust's lifetime system, lifetime annotations, and how they prevent dangling references.

## Example Code

```rust
fn main() {
    // Basic lifetime usage: returning a reference to the longer string
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'.", result);

    // Dangling reference example (uncomment to see compiler error)
    // let reference_to_nothing = dangle();
}

// Lifetime annotation: both parameters and return value must live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function would not compile, as it returns a reference to a local variable
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // Error: `s` does not live long enough
// }
```

## Key Points

- **Lifetimes** ensure that references are always valid.
- Lifetime annotations (`<'a>`) tell the compiler how long references must be valid.
- The compiler prevents dangling references at compile time.
- Most lifetimes are inferred, but explicit annotations are needed for complex cases (e.g., functions returning references).
- Returning a reference to a local variable is not allowed, as it would create a dangling reference.

## Table: Lifetime Annotation Example

| Function Signature                      | Description                                 |
|-----------------------------------------|---------------------------------------------|
| `fn longest<'a>(x: &'a str, y: &'a str)`| Both inputs and output share lifetime `'a`   |
| `fn dangle() -> &String`                | Invalid: returns reference to local value   |

---

Run the code:

```sh
cargo run --bin ch07_04_lifetime
```
