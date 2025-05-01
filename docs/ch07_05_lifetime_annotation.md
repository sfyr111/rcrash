# 7.5 Lifetime Annotations in Rust

This chapter demonstrates explicit lifetime annotations in functions, structs, and methods, and explains when and why they are needed.

## Example Code

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn summary(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }

    // Method returning a reference with the same lifetime as self
    fn title(&self) -> &'a str {
        self.title
    }
}

// Function with multiple references and explicit lifetime annotation
fn longest_with_announcement<'a>(x: &'a str, y: &'a str, ann: &str) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let b = Book { title: "Rust Book", author: "Steve" };
    println!("Book summary: {}", b.summary());
    println!("Book title: {}", b.title());

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = longest_with_announcement(s1.as_str(), s2.as_str(), "Comparing greetings");
    println!("Longest: {}", result);
}
```

## Key Points

- Lifetime annotations (e.g., `<'a>`) connect the lifetimes of references to guarantee validity.
- Structs holding references require explicit lifetime parameters.
- Methods returning references often need lifetime annotations to tie the return value to the struct's lifetime.
- Functions with multiple reference parameters may require explicit lifetimes if the relationships are not inferable.
- Lifetime annotations do **not** change how long data lives; they only describe relationships between references.

## Table: Where Lifetime Annotations Are Needed

| Scenario                          | Lifetime Annotation Required? | Example                           |
|------------------------------------|------------------------------|-----------------------------------|
| Function with one reference param  | No (elided)                  | `fn foo(x: &str)`                 |
| Function with multiple refs/return | Yes                          | `fn bar<'a>(x: &'a str, y: &'a str) -> &'a str` |
| Struct holding references          | Yes                          | `struct Foo<'a> { x: &'a str }`   |
| Method returning reference         | Often                        | `fn get(&self) -> &str`           |

---

Run the code:

```sh
cargo run --bin ch07_05_lifetime_annotation
```
