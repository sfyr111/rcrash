# Chapter 3.3: Variables, Constants, and Mutability

## Key Points

- **Variables** are immutable by default. Use `mut` to make them mutable.
- **Constants** use `const` and must have a type annotation. Their value is always fixed and set at compile time.
- **Shadowing** allows you to declare a new variable with the same name, even with a different type.

## Demo Code

```rust
const MAX_POINTS: u32 = 100_000;
println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);

let x = 5;
println!("The value of x is: {}", x);
// x = 6; // This line would cause an error because x is immutable

let mut y = 10;
println!("The initial value of y is: {}", y);
y = 20;
println!("The value of y after mutation is: {}", y);

let x = x + 1;
println!("The value of x after first shadowing (x + 1): {}", x);
let x = x * 2;
println!("The value of x after second shadowing (x * 2): {}", x);
{
    let x = x - 3;
    println!("The value of x in the inner scope after shadowing (x - 3): {}", x);
}
println!("The value of x in the outer scope after all shadowing: {}", x);
```

## Table: Variable Types and Mutability

| Type        | Example                          | Mutable? | Notes                                              |
|-------------|----------------------------------|----------|----------------------------------------------------|
| Immutable   | `let x = 5;`                    | No       | Default, cannot be changed after assignment        |
| Mutable     | `let mut y = 10;`               | Yes      | Use `mut` keyword to allow mutation                |
| Constant    | `const MAX: u32 = 100_000;`     | No       | Must specify type, always immutable, compile-time   |
| Shadowed    | `let x = x + 1;`                | Yes*     | New binding, can even change type                  |

*Shadowing creates a new variable, not a true mutation.

## How to Run

```bash
cargo run --bin ch03_3_var_mut
```

## Notes

- In **debug mode**, integer overflow will cause a panic.
- In **release mode**, integer overflow will wrap around (unless using checked/wrapping methods).
- Constants are always global and must be annotated with a type.

---

> For more details, see [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
