# Chapter 3.9: Slice Type

## Key Points

- A slice is a reference to a contiguous sequence in a collection (array, string, etc.), with a fixed length.
- Slices do not own the data; they are views into existing data.
- Array slices: `&arr[start..end]`
- String slices: `&s[start..end]` (must be valid UTF-8 boundaries)
- Mutable slices (`&mut [T]`) allow modifying the underlying data.

## Demo Code

```rust
fn main() {
    // Array slice
    let arr = [10, 20, 30, 40, 50];
    let slice_all = &arr[..];
    let slice_part = &arr[1..4];
    println!("Array: {:?}", arr);
    println!("Full slice: {:?}", slice_all);
    println!("Partial slice [1..4]: {:?}", slice_part);

    // Slices are references to a contiguous sequence
    println!("First element of slice: {}", slice_part[0]);
    println!("Length of slice: {}", slice_part.len());

    // Modifying array through mutable slice
    let mut arr2 = [1, 2, 3, 4, 5];
    let slice_mut = &mut arr2[2..];
    slice_mut[0] = 99;
    println!("Modified arr2 via slice: {:?}", arr2);

    // String slice
    let s = String::from("Hello, Rustaceans!");
    let hello = &s[0..5];
    let rest = &s[7..];
    println!("String: {}", s);
    println!("First word: {}", hello);
    println!("Rest of string: {}", rest);
}
```

## Table: Slice Features

| Feature              | Example                   | Notes                                 |
|----------------------|---------------------------|---------------------------------------|
| Array slice          | `&arr[1..4]`              | View part of an array                 |
| Full array slice     | `&arr[..]`                | View the whole array                  |
| String slice         | `&s[0..5]`                | Must be valid UTF-8 boundary          |
| Mutable slice        | `&mut arr[2..]`           | Allows modification                   |
| Access element       | `slice[0]`                | Index into the slice                  |
| Length               | `slice.len()`             | Number of elements in the slice       |

## Notes

- Slices are commonly used to work with parts of arrays and strings without copying data.
- String slices must be on valid UTF-8 boundaries, otherwise code will panic.
- Slices are read-only by default; use `&mut` for mutable access.

---

> For more details, see [Rust Book: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)