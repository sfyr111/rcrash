// ch06_04_generic_struct_impl.rs
// 6.4 Implementations for Generic Structs
// Demonstrates how to implement methods for generic structs in Rust.

/// A generic Point struct that can hold any type.
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new Point.
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// Consumes the point and returns a tuple.
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Point<f64> {
    /// Calculates the distance from the origin (only for f64).
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    println!("p1: {:?}, tuple: {:?}", p1, p1.into_tuple());

    let p2 = Point::new(1.5, 2.5);
    println!("p2: {:?}, tuple: {:?}", p2, p2.into_tuple());

    let p3 = Point::new(0.0_f64, 5.0_f64);
    println!("p3: {:?}, distance from origin: {}", p3, p3.distance_from_origin());
}
