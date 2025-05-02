// ch09_02_box.rs
// 9.2 Smart Pointer: Box<T> in Rust
// Demonstrates the basic usage, scenarios, and core motivations for Box<T>.
//
// When to use Box<T>?
// 1. When you have a type whose size is unknown at compile time, but you need to use it in a context that requires a known size (e.g., recursive types, trait objects).
// 2. When you have a large amount of data and want to transfer ownership without copying the data (heap allocation, pointer move only).
// 3. When you want to own a value and only care about whether it implements a specific trait, not its concrete type (trait objects, dynamic dispatch).
//
// Box<T> is like a "box" on the heap. You own the box (pointer), and the data stays put. Passing the box is cheap and safe.

fn main() {
    // 1. Storing data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    // Useful for large data, or when you want heap allocation explicitly.

    // 2. Recursive types (e.g., linked list)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    // Without Box, the compiler can't know the size of List at compile time.

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    // 3. Trait objects: dynamic dispatch (see ch09_03)
    // You only care about the trait, not the concrete type.
    trait Animal { fn speak(&self); }
    struct Dog;
    impl Animal for Dog { fn speak(&self) { println!("Woof"); } }
    let a: Box<dyn Animal> = Box::new(Dog);
    a.speak();

    // Box<T> is not always required, but these three cases are the most common and idiomatic.
}
