# 9.2 Smart Pointer: Box<T> in Rust

This chapter demonstrates the usage of the smart pointer `Box<T>` in Rust, including its basic features and common scenarios.

---

## When to Use Box<T>?

Box<T> is most useful in three core scenarios:

1. **When you have a type whose size is unknown at compile time, but you need to use it in a context that requires a known size**  
   *Example: recursive types, trait objects.*
   - Rust needs to know the size of every value at compile time. Recursive types (like linked lists, trees) or trait objects (`dyn Trait`) have unknown size.
   - Box<T> acts as a pointer of known size (like a "box" on the heap), so you can use them in structs, enums, etc.

2. **When you have a large amount of data and want to transfer ownership without copying the data**  
   *Example: moving large arrays, strings, or collections.*
   - Box<T> stores data on the heap. When you move a Box, only the pointer is moved, not the data itself. This is efficient and avoids expensive copies.

3. **When you want to own a value and only care about whether it implements a specific trait, not its concrete type**  
   *Example: trait objects, dynamic dispatch.*
   - Box<dyn Trait> enables dynamic dispatch, similar to Go's interface or Java's interface references. You can store any type that implements a trait, and call trait methods without knowing the concrete type.

---

## Example Code

```rust
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
    trait Animal { fn speak(&self); }
    struct Dog;
    impl Animal for Dog { fn speak(&self) { println!("Woof"); } }
    let a: Box<dyn Animal> = Box::new(Dog);
    a.speak();
}

## Key Points

- `Box<T>` is a smart pointer for allocating values on the heap.
- Typical use cases:
    1. Recursive/unknown-size types
    2. Efficient ownership transfer of large data
    3. Trait objects for dynamic dispatch
- Dereferencing a `Box<T>` gives access to the inner value.
- Boxed values are automatically dropped when they go out of scope.

---

Run the code:

```sh
cargo run --bin ch09_02_box
