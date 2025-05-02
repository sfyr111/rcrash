# 9.4 Dynamic Array: Vec<T> in Rust

This chapter demonstrates the usage, features, and best practices for the dynamic array type `Vec<T>`.

---

## When to Use Vec<T>?

Vec<T> is the most common growable collection in Rust. Use Vec<T> when:

1. **You need a collection with a variable number of elements**  
   - The size is not known at compile time or may change at runtime.
2. **You want efficient random access and push/pop at the end**  
   - Vec<T> provides O(1) access by index and O(1) push/pop at the end.
3. **You want to collect iterator results or build data at runtime**  
   - Many iterators and APIs return or accept Vec<T>.

---

## Example Code

```rust
fn main() {
    // 1. Creating a new Vec and pushing elements
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);

    // 2. Creating a Vec with initial values
    let mut v2 = vec![10, 20, 30];
    println!("v2 = {:?}", v2);

    // 3. Accessing elements by index
    println!("First element: {}", v[0]);

    // 4. Safe access with get
    match v.get(10) {
        Some(val) => println!("Element at 10: {}", val),
        None => println!("No element at index 10"),
    }

    // 5. Iterating over a Vec
    for x in &v {
        println!("element = {}", x);
    }

    // 6. Removing elements
    v.pop();
    println!("After pop: {:?}", v);

    // 7. Insert and Remove
    v2.insert(1, 99); // insert 99 at index 1
    println!("After insert: {:?}", v2);
    v2.remove(2); // remove element at index 2
    println!("After remove: {:?}", v2);

    // 8. Other useful methods
    println!("Length: {}, Is empty: {}", v2.len(), v2.is_empty());
    v2.clear();
    println!("After clear: {:?}, len = {}", v2, v2.len());

    // 9. Dedup, sort, reverse
    let mut v3 = vec![3, 1, 2, 2, 3, 1];
    v3.sort();
    v3.dedup();
    v3.reverse();
    println!("After sort, dedup, reverse: {:?}", v3);

    // 10. Retain, extend, append
    let mut v4 = vec![1, 2, 3, 4, 5];
    v4.retain(|&x| x % 2 == 1); // keep odd numbers
    println!("After retain (odd): {:?}", v4);
    v4.extend(&[7, 9]);
    println!("After extend: {:?}", v4);
    let mut v5 = vec![100, 200];
    v4.append(&mut v5);
    println!("After append: {:?}, v5: {:?}", v4, v5);

    // 11. Iter, iter_mut, into_iter
    let v6 = vec![10, 20, 30];
    for x in v6.iter() {
        print!("{} ", x);
    }
    println!("<- iter");
    let mut v7 = vec![1, 2, 3];
    for x in v7.iter_mut() {
        *x *= 2;
    }
    println!("After iter_mut: {:?}", v7);
    for x in v7.into_iter() {
        print!("{} ", x);
    }
    println!("<- into_iter");

    // 12. Capacity and reserve
    let mut v8 = Vec::with_capacity(10);
    println!("Initial capacity: {}", v8.capacity());
    v8.extend(0..5);
    println!("Capacity after extend: {}, v8 = {:?}", v8.capacity(), v8);
    v8.reserve(20);
    println!("Capacity after reserve(20): {}", v8.capacity());
}
```

## Commonly Used Vec<T> Methods

- `push`, `pop`, `insert`, `remove`, `clear`, `len`, `is_empty`
- `contains`, `dedup`, `sort`, `reverse`, `retain`, `extend`, `append`
- `iter`, `iter_mut`, `into_iter`, `drain`, `split_at`, `split_off`
- `first`, `last`, `get`, `get_mut`, `resize`, `truncate`, `capacity`, `reserve`

See the code example above for practical usage of these methods in business scenarios.

---

## Key Points

- `Vec<T>` is a growable, heap-allocated array.
- Use cases:
    1. Variable-length collections
    2. Efficient access and modification at the end
    3. Collecting iterator results
- Indexing with `[]` panics on out-of-bounds; `get` returns `Option` for safe access.
- Iteration is ergonomic and efficient.
- Removing elements from the end is O(1) (pop).

---

Run the code:

```sh
cargo run --bin ch09_04_vec
