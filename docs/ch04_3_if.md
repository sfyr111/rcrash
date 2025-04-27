# Chapter 4.3: if/else Control Flow

## Key Points

- Rust's `if` and `else` provide conditional branching, similar to other languages.
- `if` can be used as a statement (for side effects) or as an expression (returns a value).
- `else if` supports multi-branch logic.
- `if let` is a convenient pattern matching shortcut for matching a single pattern.

## Demo Code

```rust
fn main() {
    // Basic if/else
    let n = 7;
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }

    // if as an expression
    let is_even = if n % 2 == 0 { true } else { false };
    println!("{} is even? {}", n, is_even);

    // Nested if/else
    let grade = 85;
    let level = if grade >= 90 {
        "A"
    } else if grade >= 80 {
        "B"
    } else if grade >= 70 {
        "C"
    } else {
        "D or lower"
    };
    println!("grade {} -> level {}", grade, level);

    // if let (pattern matching shortcut)
    let opt = Some(10);
    if let Some(val) = opt {
        println!("if let: value is {}", val);
    } else {
        println!("if let: None");
    }
}
```

## Table: if/else Forms

| Form                 | Example                                 | Notes                                 |
|----------------------|-----------------------------------------|---------------------------------------|
| Basic if/else        | `if a { ... } else { ... }`             | Standard branching                    |
| else if chain        | `if a { ... } else if b { ... }`        | Multiple conditions                   |
| if as expression     | `let x = if cond { a } else { b };`     | Returns a value                       |
| Nested if/else       | `if ... { if ... { ... } else { ... }}` | Can nest for complex logic            |
| if let               | `if let Some(x) = opt { ... }`          | Pattern match shortcut for Option/enum|

## Notes

- All `if` branches must return the same type when used as an expression.
- `if let` is great for Option, Result, or custom enums.
- Prefer `match` for exhaustive multi-pattern matching.

---

> For more details, see [Rust Book: if Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)
