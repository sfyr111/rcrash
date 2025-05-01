// ch07_04_lifetime.rs
// 7.4 Lifetimes in Rust
// Demonstrates lifetime annotations and avoiding dangling references.

fn main() {
    // Basic lifetime usage: returning a reference to the longer string
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'.", result);

    // Dangling reference example (uncomment to see compiler error)
    // let reference_to_nothing = dangle();
}

// Lifetime annotation: both parameters and return value must live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function would not compile, as it returns a reference to a local variable
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // Error: `s` does not live long enough
// }
