# Chapter 3.13: The println! Macro

## Key Points

- `println!` is a macro for printing text and variables to the console.
- Supports formatting with `{}` (positional, named, debug, alignment, precision, etc).
- Useful for debugging and user interaction.

## Demo Code

```rust
fn main() {
    // Basic usage
    println!("Hello, world!");

    // Print variables
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);

    // Print with positional arguments
    println!("{1} is {0} years old.", age, name);

    // Print with named arguments
    println!("{person} scored {score}", person = name, score = 95);

    // Print debug info with {:?}
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);

    // Print pretty debug info with {:#?}
    println!("arr (pretty) = {:#?}", arr);

    // Print numbers in different bases
    let num = 255;
    println!("Decimal: {} Hex: {:x} Binary: {:b}", num, num, num);

    // Print with padding and alignment
    println!("|{:>5}|{:^5}|{:<5}|", 1, 2, 3); // right, center, left

    // Print floating point with precision
    let pi = 3.1415926;
    println!("pi = {:.2}", pi);
}
```

## Table: println! Features

| Feature            | Example                                       | Output / Notes                  |
|--------------------|-----------------------------------------------|---------------------------------|
| Basic print        | `println!("hi")`                              | hi                              |
| Variables          | `println!("{}", x)`                           | x value                         |
| Positional         | `println!("{1} {0}", a, b)`                   | b a                             |
| Named              | `println!("{name}", name=x)`                  | x                               |
| Debug              | `println!("{:?}", arr)`                       | arr debug format                |
| Pretty debug       | `println!("{:#?}", arr)`                      | arr pretty debug                |
| Base formatting    | `println!("{:x} {:b}", n, n)`                 | hex, binary                     |
| Padding/alignment  | `println!("|{:>5}|", n)`                      | right-align                     |
| Precision          | `println!("{:.2}", f)`                        | 2 decimal places                |

## Notes

- `println!` is a macro, not a function.
- Use `{}` for value, `{:?}` for debug, `{:x}` for hex, etc.
- For more, see [std::fmt documentation](https://doc.rust-lang.org/std/fmt/).

---

> For more details, see [Rust Book: Printing](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#printing-values-with-println)
