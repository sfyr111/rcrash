# 5.3 Visibility with `pub` in Rust

## Key Points

- By default, modules and functions in Rust are private.
- The `pub` keyword can be used to make functions, modules, or struct members public (visible outside their module).
- `pub mod` exposes a module; `pub fn` exposes a function.
- Private items are only accessible within their module and submodules; public items can be accessed from outside.

## Example Code

```rust
mod outer {
    // Private by default (implementation detail)
    fn private_fn() {
        println!("This is a private function.");
    }

    // Public function (public API)
    pub fn public_fn() {
        // inner::visible_to_outer(); // OK, because pub(super) allows parent module access
        println!("This is a public function.");
    }

    pub mod inner {
        // pub(self): Only visible within this module (rarely used; restricts API to inner module only)
        pub(self) fn only_inner() {
            println!("This is only visible in inner module (pub(self)).");
        }

        // pub(super): Visible to the parent module (outer). Useful for helpers only needed by the parent.
        pub(super) fn visible_to_outer() {
            println!("This is visible to parent module (pub(super)).");
        }

        // pub(crate): Visible anywhere in the current crate. Good for internal APIs shared across modules but not exposed outside the crate.
        pub(crate) fn visible_in_crate() {
            println!("This is visible in the whole crate (pub(crate)).");
        }

        // pub: Visible everywhere (public API)
        pub fn inner_public_fn() {
            println!("This is a public function in the inner module.");
        }

        // Private function (default): Only visible within this module and its children. Used for implementation details.
        fn inner_private_fn() {
            println!("This is a private function in the inner module.");
        }
    }
}

fn main() {
    // outer::private_fn(); // Error: function is private
    outer::public_fn(); // OK
    outer::inner::inner_public_fn(); // OK
    // outer::inner::inner_private_fn(); // Error: function is private
    // outer::inner::only_inner(); // Error: pub(self) only in inner
    // outer::inner::visible_to_outer(); // Error: pub(super) only in parent
    // outer::inner::visible_in_crate(); // OK (same crate)
    outer::inner::visible_in_crate(); // This works because we are in the same crate
}
```

## Table: pub Visibility Modifiers

| Modifier       | Visibility Scope                   | Typical Use Case                                      |
|---------------|------------------------------------|-------------------------------------------------------|
| (none/private)| Current module & submodules        | Implementation details, private helpers                |
| pub(self)     | Only current module                | Restrict API to this module (rare, explicit)           |
| pub(super)    | Parent module                      | Helpers for parent module, not public API              |
| pub(crate)    | Whole crate                        | Internal crate-wide API, not exposed to dependents     |
| pub           | Everywhere (global)                | Public API, library/external consumers                 |

## Notes

- Prefer exposing only necessary APIs and hiding implementation details.
- Struct fields are private by default; use `pub` to make them accessible from outside.
- Use different `pub` modifiers for fine-grained access control, which improves encapsulation and safety in large projects.

---

> See also: [Rust Book - Paths for Referring to an Item in the Module Tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
