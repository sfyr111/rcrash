// ch06_06_derive.rs
// 6.6 Automatic Derivation
// Demonstrates Rust's #[derive] attribute for automatic implementation of common traits.

/// A struct that automatically derives several common traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: 3, y: 4 };
    let d = Point { ..Default::default() };

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d (default): {:?}", d);

    println!("a == b: {}", a == b);
    println!("a < c: {}", a < c);
    println!("Cloned c: {:?}", c.clone());
}
