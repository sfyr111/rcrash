# 5.6 Using `super` and `self` to Simplify Module Paths

This chapter demonstrates how to use `super` and `self` in Rust to reference parent and current modules, making module paths clearer and reducing repetition.

## Key Points

- `self` refers to the current module. Useful for calling sibling functions or types.
- `super` refers to the parent module. Useful for accessing items defined one level up.
- These keywords help avoid hardcoding full paths and make refactoring easier.

## Example Code

```rust
mod outer {
    pub fn outer_fn() {
        println!("Called outer_fn()");
    }
    pub mod inner {
        pub fn inner_fn() {
            println!("Called inner_fn()");
        }
        pub fn call_outer_direct() {
            // Use super to access parent module
            super::outer_fn();
        }
        pub fn call_inner_self() {
            // Use self to call a function in the same module
            self::inner_fn();
        }
        pub fn call_both() {
            self::call_inner_self();
            super::outer_fn();
        }
    }
}

fn main() {
    // Call from main
    outer::outer_fn();
    outer::inner::inner_fn();
    outer::inner::call_outer_direct();
    outer::inner::call_inner_self();
    outer::inner::call_both();
}
```

## Table: `self` and `super` Usage

| Keyword | Example                      | Description                                  |
|---------|------------------------------|----------------------------------------------|
| self    | `self::foo()`                | Calls `foo` in current module                |
| super   | `super::bar()`               | Calls `bar` in parent module                 |

## Notes

- Prefer `self` and `super` for relative paths inside modules.
- Refactoring module names is easier with relative paths.
- These keywords only work within module scopes.
