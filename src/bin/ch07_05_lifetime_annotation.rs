// ch07_05_lifetime_annotation.rs
// 7.5 Lifetime Annotations in Rust
// Demonstrates explicit lifetime annotations in functions, structs, and methods.

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
