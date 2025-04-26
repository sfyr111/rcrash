# Chapter 3.12: Comment Types

## Key Points

- Rust supports several types of comments:
    - Single-line: `// ...`
    - Multi-line: `/* ... */`
    - Documentation (doc): `/// ...` 
- Use comments to explain code, disable code, or generate documentation.

## Demo Code

```rust
fn main() {
    // Single-line comment: use // for normal comments
    let x = 5; // This is an inline comment

    /*
        Multi-line comment: use /* ... */
        Good for temporarily disabling code or writing longer notes.
    */
    let y = 10;

    /// Documentation comment: use triple slashes (///)
    /// This function adds two numbers.
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sum = add(x, y);
    println!("sum = {}", sum);

    println!("x = {}, y = {}", x, y); // Output variables
}
```

## Table: Comment Types

| Type                | Syntax             | Usage                                      |
|---------------------|--------------------|---------------------------------------------|
| Single-line         | `// comment`       | Normal code explanation, inline             |
| Multi-line          | `/* comment */`    | Block comments, disable code                |
| Doc (outer)         | `/// comment`      | Generates docs for items (fn, struct, etc.) |
| Doc (inner)         | `//! comment`      | Docs for enclosing module or crate          |

## Notes

- Use comments for clarity, not to restate obvious code.
- Documentation comments (`///`, `//!`) are used by `cargo doc` to generate HTML docs.
- Too many comments may indicate unclear code; prefer clear code plus concise comments.

---

> For more details, see [Rust Book: Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
