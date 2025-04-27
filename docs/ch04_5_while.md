# Chapter 4.5: Using while

## Key Points

- `while` loops run as long as a condition is true.
- Useful for countdowns, iterating with indices, or when the number of iterations is not known in advance.
- `while let` is handy for pattern matching in loops (e.g., Option/enum).

## Demo Code

```rust
fn main() {
    // Basic while loop
    let mut n = 3;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }
    println!("Liftoff!");

    // while loop over a collection (using index)
    let arr = [10, 20, 30, 40];
    let mut idx = 0;
    while idx < arr.len() {
        println!("arr[{}] = {}", idx, arr[idx]);
        idx += 1;
    }

    // while let for Option/enum pattern matching
    let mut opt = Some(5);
    while let Some(x) = opt {
        println!("while let: x = {}", x);
        if x == 0 {
            opt = None;
        } else {
            opt = Some(x - 1);
        }
    }
    println!("while let finished");
}
```

## Table: while Forms

| Form                | Example                          | Notes                                  |
|---------------------|----------------------------------|----------------------------------------|
| Basic while         | `while cond { ... }`             | Runs while cond is true                |
| Indexed collection  | `while idx < arr.len() { ... }`  | Manual index iteration                 |
| while let pattern   | `while let Some(x) = opt { ... }`| Loop with pattern matching (Option etc) |

## Notes

- Use `while` when the number of iterations is unknown or depends on runtime conditions.
- For known ranges, prefer `for` loops.
- `while let` is concise for Option/Result/enum value extraction.

---

> See also: [Rust Book: while Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-a-while-loop)
