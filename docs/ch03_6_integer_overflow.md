# Chapter 3.6: Integer Overflow

## Key Points

- Integer overflow occurs when a value exceeds the range of its type.
- Rust provides several methods to handle overflow: `wrapping_*`, `checked_*`, `overflowing_*`, and `saturating_*`.
- In debug mode, plain arithmetic overflow panics; in release mode, it wraps around.

## Demo Code

```rust
let max_u8 = std::u8::MAX;
println!("The maximum value of u8 is: {}", max_u8);
println!("max_u8.wrapping_add(1) = {} (wraps around to 0)", max_u8.wrapping_add(1));
println!("max_u8.checked_add(1) = {:?} (None means overflow)", max_u8.checked_add(1));
let (overflowing, did_overflow) = max_u8.overflowing_add(1);
println!("max_u8.overflowing_add(1) = {} (overflowed: {})", overflowing, did_overflow);
println!("max_u8.saturating_add(1) = {} (saturates at max)", max_u8.saturating_add(1));

let min_i8 = std::i8::MIN;
println!("The minimum value of i8 is: {}", min_i8);
println!("min_i8.wrapping_sub(1) = {} (wraps around to 127)", min_i8.wrapping_sub(1));
let (overflowing_i8, did_overflow_i8) = min_i8.overflowing_sub(1);
println!("min_i8.overflowing_sub(1) = {} (overflowed: {})", overflowing_i8, did_overflow_i8);
println!("min_i8.saturating_sub(1) = {} (saturates at min)", min_i8.saturating_sub(1));
```

## Table: Integer Overflow Handling Methods

| Method             | Behavior on Overflow      | Return Type         | Example                          |
|--------------------|--------------------------|---------------------|-----------------------------------|
| `wrapping_add`     | Wraps around             | Value               | `255u8.wrapping_add(1) // 0`      |
| `checked_add`      | Returns `None`           | Option<Value>       | `255u8.checked_add(1) // None`    |
| `overflowing_add`  | Returns (value, bool)    | (Value, overflowed) | `255u8.overflowing_add(1) // (0, true)` |
| `saturating_add`   | Clamps to max/min        | Value               | `255u8.saturating_add(1) // 255`  |

## How to Run

```bash
cargo run --bin ch03_6_integer_overflow
```

## Notes

- Use `checked_*` for safety-critical code.
- Use `saturating_*` when you want to clamp at the boundary.
- Use `wrapping_*` or `overflowing_*` for low-level or performance-sensitive code.

---

> For more details, see [Rust Book: Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
