# Chapter 3.8: Array and String Indexing

## Key Points

- An array is a fixed-size collection of elements of the same type.
- Arrays are stored on the stack and their length is known at compile time.
- Array elements are accessed by index (zero-based).
- Arrays can be iterated, sliced, mutated, and initialized with repeated values.
- You can use a string as an index by parsing it to a number (with error/out-of-bounds handling).
- Strings in Rust cannot be indexed directly by integer; use slicing (on valid boundaries) or `.chars().nth()` for character access.

## Demo Code

```rust
fn main() {
    // Creating an array with explicit type and length
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

    // Accessing array elements by index
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[4]);

    // Getting array length
    println!("Array length: {}", arr.len());

    // Iterating over array elements
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, val);
    }

    // Initializing an array with the same value
    let zeros = [0; 8];
    println!("Array of zeros: {:?}", zeros);

    // Slicing an array
    let slice = &arr[1..4];
    println!("Slice of arr[1..4]: {:?}", slice);

    // Using a computed value as an index
    let idx = 2 + 1; // Example: computed index
    println!("Element at computed index {}: {}", idx, arr[idx]);

    // Mutating array elements using a computed index
    let update_idx = arr.len() - 1;
    arr[update_idx] = 42;
    println!("After mutation, arr: {:?}", arr);

    // Using a string as an index (parse string to usize), step by step
    let str_index = "2";
    let parsed = str_index.parse::<usize>();
    if parsed.is_ok() {
        let i = parsed.unwrap();
        if i < arr.len() {
            println!("arr[{}] (from string index) = {}", str_index, arr[i]);
        } else {
            println!("Index {} out of bounds!", i);
        }
    } else {
        println!("Failed to parse index '{}': {}", str_index, parsed.unwrap_err());
    }

    // String indexing and slicing
    let s = String::from("hello, ");
    println!("String: {}", s);
    // String slicing (must be on valid char boundaries)
    let sub = &s[0..5]; // 'hello'
    println!("First 5 bytes as slice: {}", sub);
    // Attempting to index a string directly is not allowed:
    // let ch = s[1]; // This line would not compile
    // Instead, use chars().nth()
    let nth = s.chars().nth(1);
    if nth.is_some() {
        println!("Second character: {}", nth.unwrap());
    } else {
        println!("No second character");
    }
}
```

## Table: Array and String Features

| Feature                | Example                        | Notes                                              |
|------------------------|--------------------------------|----------------------------------------------------|
| Create array           | `[1, 2, 3]`                    | All elements must be the same type                 |
| Fixed size             | `[i32; 5]`                     | Length known at compile time                       |
| Indexing (array)       | `arr[0]`, `arr[idx]`           | Zero-based indexing, idx can be computed           |
| Index by string        | `let parsed = "2".parse::<usize>()` <br> `if parsed.is_ok() { arr[parsed.unwrap()] }` | Parse string to number, handle errors, use if/else |
| Mutate (array)         | `arr[2] = 99`                  | Arrays must be mutable                             |
| Length                 | `arr.len()`                    | Returns the array length                           |
| Iterate                | `for x in arr.iter()`          | Access each element                                |
| Initialize with value  | `[0; 8]`                       | 8 elements, all 0                                  |
| Slice (array)          | `&arr[1..4]`                   | Borrow part of the array                           |
| String slicing         | `&s[0..5]`                     | Must be valid UTF-8 boundary                       |
| String char access     | `let nth = s.chars().nth(1)` <br> `if nth.is_some() { nth.unwrap() }` | Use if/else for Option<char>                       |
| Not allowed            | `s[1]`                         | Direct string indexing is not allowed in Rust      |

## How to Run

```bash
cargo run --bin ch03_8_array
```

## Notes

- Accessing an out-of-bounds index will panic at runtime.
- Arrays are different from vectors (`Vec<T>`), which are growable and heap-allocated.
- Use `.parse::<usize>()` to convert a string to an index, and always check for errors and bounds, using if/else for beginners.
- String slices must be on valid UTF-8 boundaries, otherwise code will panic.
- Use `.chars().nth(n)` and if/else for character access in Unicode strings.

---

> For more details, see [Rust Book: The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) and [Rust Book: The String Type](https://doc.rust-lang.org/book/ch08-02-strings.html)
