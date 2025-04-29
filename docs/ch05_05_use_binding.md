# 5.5 Using `use` to Bind Module Members

This chapter demonstrates how to use the `use` keyword in Rust to bring module members (functions, constants, types) into scope, making code more concise and readable.

## Key Points

- `use` allows you to import functions, types, or constants from modules.
- You can rename imports using `as`.
- Importing makes calling items shorter and improves code clarity.

## Example Code

```rust
mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner::greet()");
        }
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        pub const MAGIC: i32 = 42;
    }
}

use outer::inner::greet;
use outer::inner::add as add_fn;
use outer::inner::MAGIC;

fn main() {
    greet();
    let sum = add_fn(3, 4);
    println!("add_fn(3, 4) = {}", sum);
    println!("MAGIC = {}", MAGIC);
    let sum2 = outer::inner::add(10, 20);
    println!("outer::inner::add(10, 20) = {}", sum2);
}
```

## Table: Typical `use` Patterns

| Pattern                      | Example                              | Description                             |
|------------------------------|--------------------------------------|-----------------------------------------|
| Import function              | `use foo::bar;`                      | Brings `bar` into scope                 |
| Import and rename            | `use foo::bar as baz;`               | Use `baz` instead of `bar`              |
| Import multiple items        | `use foo::{bar, baz};`               | Multiple imports in one line            |
| Import all public items      | `use foo::*;`                        | Glob import (not recommended for libs)  |

## Notes

- Prefer explicit imports for clarity.
- `use` only brings public items into scope.
- Use renaming (`as`) to avoid name conflicts.
- Avoid glob imports (`*`) in libraries for better maintainability.
