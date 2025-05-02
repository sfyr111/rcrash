// ch09_03_rc.rs
// 9.3 Reference Counting: Rc<T> in Rust
// Demonstrates the usage, motivation, and pitfalls of Rc<T>.
//
// When to use Rc<T>?
// 1. When you want multiple parts of your program to share ownership of some data (e.g., graph, tree with shared nodes).
// 2. When compile-time ownership rules (single owner) are too strict for your data structure.
// 3. Rc<T> is for single-threaded scenarios. For multi-threaded, use Arc<T>.
//
// Rc<T> keeps track of the number of references, and only deallocates the data when the last Rc goes out of scope.

use std::rc::Rc;

fn main() {
    // Example 1: Simple shared ownership
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
    println!("Reference count after clone: {}", Rc::strong_count(&a));

    // Example 2: Shared nodes in a list
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let tail = Rc::new(Cons(10, Rc::new(Nil)));
    let list1 = Cons(5, Rc::clone(&tail));
    let list2 = Cons(3, Rc::clone(&tail));
    println!("list1 = {:?}\nlist2 = {:?}", list1, list2);
    println!("Tail reference count: {}", Rc::strong_count(&tail));

    // Rc<T> does not provide interior mutability or thread safety.
    // For mutability, see RefCell<T> or Cell<T>.
    // For thread safety, use Arc<T>.
}
