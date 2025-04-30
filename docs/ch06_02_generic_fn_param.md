# 6.2 Generics as Function Parameter Types

This chapter demonstrates how to use generics as function parameter types in Rust. Generics enable writing flexible and reusable code that works for multiple data types.

## Example: Generic Functions

```rust
/// Returns the larger of two values.
/// Works for any type that implements the PartialOrd and Copy traits.
pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// Swaps two values in a tuple.
pub fn swap<T>(pair: (T, T)) -> (T, T) {
    let (x, y) = pair;
    (y, x)
}

fn main() {
    let a = 10;
    let b = 20;
    println!("max({}, {}) = {}", a, b, max(a, b));

    let x = 3.14;
    let y = 2.71;
    println!("max({}, {}) = {}", x, y, max(x, y));

    let pair = ("hello", "world");
    let swapped = swap(pair);
    println!("swap({:?}) = {:?}", pair, swapped);
}
```

## Key Points

- Use `<T>` to declare a generic type parameter for a function.
- Trait bounds (e.g., `T: PartialOrd + Copy`) restrict the types that can be used.
- Generics increase code reuse and type safety.

## Table: Comparison

| Function        | Description                                   |
|----------------|-----------------------------------------------|
| `max<T>`       | Returns the larger value of two of type `T`    |
| `swap<T>`      | Swaps two values of type `T` in a tuple        |

## Notes

- Generic functions are monomorphized at compile time for each concrete type used.
- Rust requires explicit trait bounds for operations like comparison or copying.
