# Chapter 4.13: High-Order Functions

## Key Points

- High-order functions take one or more functions/closures as arguments or return them as results.
- Common in iterator combinators: `map`, `filter`, `fold`, etc.
- Enable concise, expressive, and composable code.

## Demo Code

```rust
// Demonstration of high-order functions in Rust

fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn main() {
    // Pass closure as argument
    let square = |x| x * x;
    let result = apply_twice(square, 2);
    println!("apply_twice(square, 2) = {}", result); // (2*2)*2*2 = 16

    // Use map, filter, fold (high-order methods on iterators)
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("squares = {:?}", squares);

    let even: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("even numbers = {:?}", even);

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    // Returning a function (closure)
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    let triple = make_multiplier(3);
    println!("triple(6) = {}", triple(6));
}
```

## Table: High-Order Function Patterns

| Pattern                      | Example                                      | Description                                         |
|------------------------------|----------------------------------------------|-----------------------------------------------------|
| Function as argument         | `apply_twice(square, 2)`                     | Pass closure/function to another function           |
| Iterator combinator: map     | `numbers.iter().map(|x| x * x)`              | Transform each element                             |
| Iterator combinator: filter  | `numbers.iter().filter(|x| **x % 2 == 0)`    | Keep elements matching a predicate                  |
| Iterator combinator: fold    | `numbers.iter().fold(0, |acc, x| acc + x)`   | Accumulate values                                  |
| Return function/closure      | `make_multiplier(3)`                         | Return a closure from a function                   |

## Notes

- High-order functions are widely used in Rust for functional-style programming.
- They improve code clarity and reduce boilerplate.
- Iterator combinators are lazy and chainable.
- Use `move` keyword in closures to capture environment by value.

---

> See also: [Rust Book: Closures and High-Order Functions](https://doc.rust-lang.org/book/ch13-01-closures.html)
