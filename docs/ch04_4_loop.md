# Chapter 4.4: Using loop

## Key Points

- `loop` creates an infinite loop unless explicitly broken.
- Can be used as an expression: `break` can return a value.
- Supports nested loops with labels for fine-grained control.

## Demo Code

```rust
fn main() {
    // Basic loop: infinite unless break
    let mut count = 0;
    loop {
        count += 1;
        println!("count = {}", count);
        if count == 3 {
            println!("Break at count = 3");
            break;
        }
    }

    // loop as an expression (returns value with break)
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 5 {
            break n * 10; // returns 50
        }
    };
    println!("Result from loop = {}", result);

    // Nested loop with labels
    let mut outer = 0;
    'outer_loop: loop {
        let mut inner = 0;
        loop {
            println!("outer={}, inner={}", outer, inner);
            if inner == 2 { break; }
            if outer == 1 { break 'outer_loop; }
            inner += 1;
        }
        outer += 1;
    }
    println!("Exited nested loop");
}
```

## Table: loop Forms

| Form                      | Example                  | Notes                                 |
|---------------------------|--------------------------|---------------------------------------|
| Basic loop                | `loop { ... }`           | Infinite unless break                 |
| loop as expression        | `let x = loop { break y; }` | Returns value with break          |
| Labeled nested loops      | `'label: loop { ... }`   | Control which loop to break/continue  |

## Notes

- Use `break` to exit a loop, optionally returning a value.
- Use labels for clarity in nested loop control.
- For known iteration counts, prefer `for` or `while` loops.

---

> See also: [Rust Book: Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)
