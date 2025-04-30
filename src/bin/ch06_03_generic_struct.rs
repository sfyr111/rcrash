// ch06_03_generic_struct.rs
// 6.3 Generics in Structs
// Demonstrates how to use generics in Rust structs.

/// A generic Point struct that can hold any type.
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// A generic Pair struct with two different types.
#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

fn main() {
    let int_point = Point { x: 3, y: 4 };
    let float_point = Point { x: 1.2, y: 3.4 };
    let string_point = Point { x: "hello", y: "world" };

    println!("int_point: {:?}", int_point);
    println!("float_point: {:?}", float_point);
    println!("string_point: {:?}", string_point);

    let pair = Pair { first: 42, second: "answer" };
    println!("pair: {:?}", pair);
}
