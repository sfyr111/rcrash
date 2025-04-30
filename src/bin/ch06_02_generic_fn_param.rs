// ch06_02_generic_fn_param.rs
// 6.2 Generics as Function Parameter Types
// Demonstrates how to use generics as function parameter types in Rust.

/// Returns the larger of two values.
/// Works for any type that implements the PartialOrd and Copy traits.
pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// Swaps two values in a tuple.
pub fn swap<T>(pair: (T, T)) -> (T, T) {
    let (x, y) = pair;
    (y, x)
}

fn main() {
    let a = 10;
    let b = 20;
    println!("max({}, {}) = {}", a, b, max(a, b));

    let x = 3.14;
    let y = 2.71;
    println!("max({}, {}) = {}", x, y, max(x, y));

    let pair = ("hello", "world");
    let swapped = swap(pair);
    println!("swap({:?}) = {:?}", pair, swapped);
}
