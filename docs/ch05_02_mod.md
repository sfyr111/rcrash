# 5.2 Modular Programming in Rust

## Key Points

- Rust modules are declared with `mod` and used to organize code, control visibility, and encapsulate implementation details.
- By default, items in a module are private; use `pub` to make them public.
- Modules can be nested to form a hierarchy.
- Use `::` syntax to access public items in modules.
- Private items are accessible only within their module and submodules.

## Example Code

```rust
// ch05_02_mod.rs
// Demonstration of modular programming in Rust
// Showcases module declaration, visibility, and usage.

mod math {
    // Private function (default)
    fn private_add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Public function
    pub fn add(a: i32, b: i32) -> i32 {
        private_add(a, b)
    }

    // Nested module
    pub mod advanced {
        pub fn square(x: i32) -> i32 {
            x * x
        }
    }
}

fn main() {
    // Using public function from module
    let sum = math::add(2, 3);
    println!("2 + 3 = {}", sum);

    // Using public function from nested module
    let squared = math::advanced::square(4);
    println!("4 squared = {}", squared);

    // The following line would not compile (private function):
    // let _ = math::private_add(2, 3);
}
```

## Table: Module Visibility Comparison

| Scope                | Access Private Items | Access Public Items |
|----------------------|:-------------------:|:------------------:|
| Current module       | ✅                  | ✅                 |
| Submodules           | ✅                  | ✅                 |
| Parent/external code | ❌                  | ✅                 |

## Notes

- It is recommended to group related functionality into modules for better maintainability and reusability.
- Use `pub mod` and `pub fn` to expose APIs step by step, hiding implementation details.
- In larger Rust projects, modules can be split across multiple files using the `mod.rs` or directory structure.

---

> See also: [Rust Book - Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
