# Chapter 3.7: Tuple

## Key Points

- A tuple is a fixed-size collection of values of potentially different types.
- Tuple elements are accessed by position (zero-based index).
- Tuples can be destructured into individual variables.
- Parentheses with a trailing comma are required for single-element tuples.
- Tuples can be nested.
- Functions can return tuples, which is a common way to return multiple values in Rust.

## Demo Code

```rust
fn return_tuple() -> (i32, f64, &'static str) {
    (7, 3.14, "tuple from function")
}

fn main() {
    // Creating a tuple with different types
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    println!("Tuple: {:?}", tup);

    // Accessing tuple elements by index
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("First: {}, Second: {}, Third: {}", first, second, third);

    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("Destructured: x = {}, y = {}, z = {}", x, y, z);

    // Nested tuple
    let nested = ((1, 2), (3.0, false));
    println!("Nested tuple: {:?}", nested);

    // Single-element tuple vs. value
    let single = (42,);
    let not_a_tuple = 42;
    println!("Single-element tuple: {:?}", single);
    println!("Not a tuple, just an integer: {}", not_a_tuple);

    // Function returning a tuple
    let result = return_tuple();
    println!("Function returned tuple: {:?}", result);
    let (a, b, c) = return_tuple();
    println!("Destructured returned tuple: a = {}, b = {}, c = {}", a, b, c);
}
```

## Table: Tuple Features

| Feature                    | Example                      | Notes                                 |
|----------------------------|------------------------------|---------------------------------------|
| Create tuple               | `(1, 2.0, 'a')`              | Types can differ                      |
| Access by index            | `tup.0`, `tup.1`             | Zero-based indexing                   |
| Destructuring              | `let (a, b, c) = tup;`       | Assigns each element to a variable    |
| Nested tuple               | `((1, 2), (3.0, false))`     | Tuples can contain tuples             |
| Single-element tuple       | `(42,)`                      | Trailing comma is required            |
| Not a tuple (just a value) | `42`                         | No comma, just an integer             |
| Function return            | `fn foo() -> (i32, f64)`     | Common for returning multiple values  |

## How to Run

```bash
cargo run --bin ch03_7_tuple
```

## Notes

- Tuples are useful for returning multiple values from a function.
- Tuple size and types are fixed at compile time.
- Tuples implement the `Debug` trait, so you can print them with `{:?}`.

---

> For more details, see [Rust Book: The Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
