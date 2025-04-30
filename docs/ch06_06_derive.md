# 6.6 Automatic Derivation

This chapter demonstrates Rust's `#[derive]` attribute for automatically implementing common traits for your types.

## Example: Using `#[derive]`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: 3, y: 4 };
    let d = Point { ..Default::default() };

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d (default): {:?}", d);

    println!("a == b: {}", a == b);
    println!("a < c: {}", a < c);
    println!("Cloned c: {:?}", c.clone());
}
```

## Key Points

- The `#[derive]` attribute automatically implements traits like `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Default` for your struct or enum.
- This saves boilerplate and ensures consistency.
- Derived traits require all fields to also implement those traits.

## Table: Derived Traits

| Trait         | Purpose                              |
|-------------- |--------------------------------------|
| Debug         | Enables `{:?}` formatting            |
| Clone, Copy   | Value duplication                    |
| PartialEq, Eq | Equality comparison                  |
| PartialOrd, Ord | Ordering comparisons                |
| Default       | Provides a default value             |

## Notes

- You can combine multiple traits in one `#[derive(...)]`.
- For custom logic, you can manually implement a trait instead of deriving it.
