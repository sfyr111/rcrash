# Chapter 4.11: Functions and Closures

## Key Points

- Functions are defined with `fn`, closures are anonymous functions that can capture their environment.
- Closures use `|args| expr` syntax, support type inference, and can be stored in variables.
- Closures can be passed as arguments, returned from functions, and can capture variables from their scope.
- Three closure traits: `Fn`, `FnMut`, `FnOnce` (based on how they capture environment).

## Demo Code

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}

fn main() {
    // Regular function
    let sum = add(2, 3);
    println!("add(2, 3) = {}", sum);

    // Closure: basic usage
    let closure_add = |a: i32, b: i32| a + b;
    println!("closure_add(2, 3) = {}", closure_add(2, 3));

    // Closure: type inference
    let multiply = |x, y| x * y;
    println!("multiply(4, 5) = {}", multiply(4, 5));

    // Closure capturing environment
    let offset = 10;
    let add_offset = |x| x + offset;
    println!("add_offset(5) = {}", add_offset(5));

    // Passing closure to function
    let result = apply(|x, y| x - y, 8, 3);
    println!("apply(|x, y| x - y, 8, 3) = {}", result);

    // Returning closure (requires Box)
    fn make_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |b| a + b)
    }
    let adder = make_adder(100);
    println!("adder(23) = {}", adder(23));
}
```

## Table: Functions vs Closures

| Feature                | Function                | Closure                              |
|------------------------|------------------------|--------------------------------------|
| Syntax                 | `fn add(a, b) {}`      | `let c = |a, b| expr;`               |
| Type inference         | No                     | Yes                                  |
| Environment capture    | No                     | Yes (by ref, mut, or move)           |
| Can be stored/returned | Yes (fn pointer)       | Yes (with generics or trait object)  |
| Traits                 | `Fn` pointer           | `Fn`, `FnMut`, `FnOnce`              |

## Notes

- Use closures for short, inline logic, especially with iterators and callbacks.
- Use `move` keyword to force environment capture by value.
- Returning closures requires boxing (`Box<dyn Fn()>`) due to size/type.
- Fn traits:
    - `Fn` (by reference, no mutation)
    - `FnMut` (by mutable reference)
    - `FnOnce` (by value, can consume environment)

---

> See also: [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
