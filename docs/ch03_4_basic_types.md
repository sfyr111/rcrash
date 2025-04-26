# Chapter 3.4: Basic Data Types

## Key Points

- Rust has several built-in basic types: integers, floating-point numbers, booleans, characters, tuples, arrays, slices, and strings.
- Integer and floating-point literals have default types, but you can specify types with suffixes or annotations.
- Strings in Rust come in two main forms: `&str` (string slice) and `String` (owned string).
- Slices allow referencing a part of an array or string without copying.

## Demo Code

```rust
// Integer types
let a = -42; // i32 by default
let b = 42u32; // explicitly u32 using suffix
let c = 1_000_000_000i64; // explicitly i64 using suffix
let d: u8 = 255; // explicitly u8 using annotation

// Floating-point types
let x = 3.14; // f64 by default
let y = 2.71828f32; // explicitly f32 using suffix
let z: f32 = 1.41421; // explicitly f32 using annotation

// Floating-point precision demo
let f64_long = 1.2345678901234567890_f64;
let f32_long = 1.2345678901234567890_f32;

// Boolean type
let is_active = true;

// Character type
let letter = 'A';
let emoji = '😄';

// String types
let s1 = "Hello"; // &str
let s2 = String::from("World"); // String
let long_str = "123456789012345678901234567890";

// Tuple type
let tup = (500, 6.4, 1); // (i32, f64, i32)
let (tup_x, tup_y, tup_z) = tup;

// Array type
let arr = [1, 2, 3]; // [i32; 3] by default

// Slice type
let slice = &arr[1..]; // &[i32]
```

## Table: Basic Types in Rust

| Type          | Example                        | Default Type | Notes                                  |
|---------------|-------------------------------|--------------|----------------------------------------|
| Integer       | `let a = 1;`                  | i32          | Suffix: `1u8`, `1i64`, etc.            |
| Float         | `let x = 3.14;`               | f64          | Suffix: `3.14f32`                      |
| Boolean       | `let b = true;`               | bool         | Only `true` or `false`                 |
| Character     | `let c = 'A';`                | char         | 4 bytes, Unicode                       |
| Tuple         | `let t = (1, 2.0, 'a');`      | inferred     | Fixed size, can mix types              |
| Array         | `let arr = [1, 2, 3];`        | [i32; 3]     | Fixed size, same type                  |
| Slice         | `let s = &arr[1..];`          | &[i32]       | Reference to part of array or string   |
| String Slice  | `let s = "abc";`             | &str         | Immutable, fixed in binary             |
| String        | `let s = String::from("a");` | String       | Growable, heap-allocated               |

## Floating-Point Precision Example

| Expression                  | Value (f64)                  | Value (f32)      |
|-----------------------------|------------------------------|------------------|
| `1.2345678901234567890_f64` | 1.23456789012345669043...    | -                |
| `1.2345678901234567890_f32` | -                            | 1.2345678806304932|

## How to Run

```bash
cargo run --bin ch03_4_basic_types
```

## Notes

- Use type suffixes or annotations for non-default types.
- Slices and string slices are references, not owned data.
- Characters are Unicode and can include emoji.

---

> For more details, see [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
