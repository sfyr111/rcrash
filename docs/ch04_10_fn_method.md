# Chapter 4.10: Functions and Methods

## Key Points

- Rust uses the `fn` keyword to define functions.
- Functions have parameters and return values, and types must be explicitly declared.
- Methods are defined in an `impl` block, with the first parameter usually being `self` to operate on struct instances.
- Associated functions (no `self` parameter) are like static methods.

## Demo Code

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_greeting(name: &str) {
    println!("Hello, {}!", name);
}

struct Counter {
    value: i32,
}

impl Counter {
    // Associated function (like a static method)
    fn new(start: i32) -> Self {
        Counter { value: start }
    }
    // Method (has self parameter)
    fn inc(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    // Regular function
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    print_greeting("Rustacean");

    // Methods and associated function
    let mut c = Counter::new(10);
    c.inc();
    c.inc();
    println!("Counter value = {}", c.get());
}
```

## Table: Function and Method Usage

| Usage           | Example                           | Description                  |
|-----------------|-----------------------------------|------------------------------|
| Regular function| `fn add(a: i32, b: i32)`          | Define a free function       |
| Return value    | `-> i32`                          | Explicit type, `return` optional |
| Associated func | `fn new(...) -> Self`             | In `impl`, no self param     |
| Method          | `fn inc(&mut self)`               | In `impl`, has self param    |
| Call method     | `c.inc()`                         | Call with instance           |
| Call assoc func | `Counter::new(10)`                | Call with type name          |

## Notes

- Rust does not support function overloading.
- Methods must be defined within an `impl` block.
- `&self` means immutable borrow, `&mut self` means mutable borrow.
- Associated functions are often used as constructors (like `new`).

---

> See also: [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
> See also: [Rust Book: Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
