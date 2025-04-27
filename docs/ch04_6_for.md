# Chapter 4.6: Using for/range

## Key Points

- `for` loops are the preferred way to iterate in Rust.
- Can iterate over ranges, arrays, slices, strings, and any iterator.
- `for i in 0..n` is an exclusive range; `for i in 0..=n` is inclusive.
- Use `.enumerate()` for index + value pairs.

## Demo Code

```rust
fn main() {
    // for with range (exclusive end)
    for i in 0..5 {
        println!("i = {}", i);
    }
    println!("---");
    // for with range (inclusive end)
    for i in 1..=3 {
        println!("i = {}", i);
    }
    println!("---");
    // for over array
    let arr = [10, 20, 30];
    for val in arr.iter() {
        println!("val = {}", val);
    }
    println!("---");
    // for with enumerate (index + value)
    for (idx, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", idx, val);
    }
    println!("---");
    // for over chars in a string
    let s = "Rust";
    for c in s.chars() {
        println!("char = {}", c);
    }
}
```

## Table: for Loop Forms

| Form                       | Example                         | Notes                                 |
|----------------------------|---------------------------------|---------------------------------------|
| Range (exclusive)          | `for i in 0..n {}`              | i = 0 to n-1                          |
| Range (inclusive)          | `for i in 0..=n {}`             | i = 0 to n                            |
| Array/slice                | `for x in arr.iter() {}`        | Iterate over values                   |
| With enumerate             | `for (i, x) in arr.iter().enumerate()` | Index + value                |
| String chars               | `for c in s.chars() {}`         | Iterate over Unicode characters       |

## Notes

- `for` is safer and more idiomatic than manual while/index loops.
- Works with any type that implements the `IntoIterator` trait.
- Prefer `for` for most iteration needs in Rust.

---

> See also: [Rust Book: for Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
