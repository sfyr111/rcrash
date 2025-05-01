// ch07_03_borrowing.rs
// 7.3 Borrowing in Rust
// Demonstrates immutable and mutable borrowing in Rust.

fn main() {
    // Immutable borrow
    let s = String::from("hello");
    let len = calculate_length(&s); // &s is an immutable reference
    println!("The length of '{}' is {}.", s, len); // s is still valid

    // Multiple immutable borrows are allowed
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // Mutable borrow
    let mut s2 = String::from("world");
    change(&mut s2); // &mut s2 is a mutable reference
    println!("After change: {}", s2);

    // Only one mutable borrow at a time
    // let r3 = &mut s2;
    // let r4 = &mut s2; // Error: cannot borrow `s2` as mutable more than once at a time

    // Cannot mix mutable and immutable borrows
    // let r5 = &s2;
    // let r6 = &mut s2; // Error: cannot borrow `s2` as mutable because it is also borrowed as immutable
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", borrow!");
}
