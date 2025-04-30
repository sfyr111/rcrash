# 6.3 Generics in Structs

This chapter demonstrates how to use generics in Rust structs, enabling flexible and reusable data structures.

## Example: Generic Structs

```rust
/// A generic Point struct that can hold any type.
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// A generic Pair struct with two different types.
#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

fn main() {
    let int_point = Point { x: 3, y: 4 };
    let float_point = Point { x: 1.2, y: 3.4 };
    let string_point = Point { x: "hello", y: "world" };

    println!("int_point: {:?}", int_point);
    println!("float_point: {:?}", float_point);
    println!("string_point: {:?}", string_point);

    let pair = Pair { first: 42, second: "answer" };
    println!("pair: {:?}", pair);
}
```

## Key Points

- Use `<T>` or `<T, U>` to declare generic parameters for structs.
- Generics allow structs to store any type, increasing code reuse.
- You can derive traits like `Debug`, `Clone`, and `Copy` for generic structs (if the type parameters also implement them).

## Table: Comparison

| Struct         | Description                                 |
|---------------|---------------------------------------------|
| `Point<T>`    | 2D point with both fields of type `T`        |
| `Pair<T, U>`  | Pair of two possibly different types         |

## Notes

- Generic structs are instantiated with concrete types at compile time.
- You can add methods with generic or concrete type constraints.
