# 9.6 Multiple String Types in Rust

This chapter demonstrates the usage, differences, and best practices for Rust's major string types.

---

## Core String Types

1. **&str**: String slice, immutable view into a UTF-8 string, usually string literals or slices.
2. **String**: Growable, heap-allocated UTF-8 string.
3. **OsString/OsStr**: For platform-native strings (file paths, etc).
4. **CString/CStr**: For interoperability with C APIs (null-terminated).

---

## Example Code

```rust
use std::ffi::{CString, CStr, OsString, OsStr};

fn main() {
    // 1. &str: string slice
    let s1: &str = "Hello, world!";
    println!("&str: {}", s1);

    // 2. String: owned, growable string
    let mut s2 = String::from("Hello");
    s2.push(',');
    s2.push_str(" world");
    println!("String: {}", s2);

    // 3. Converting between &str and String
    let s3: String = s1.to_string();
    let s4: &str = &s2;
    println!("Converted: s3 = {}, s4 = {}", s3, s4);

    // 4. String concatenation
    let s5 = s2.clone() + "!";
    let s6 = format!("{} {}", s1, s3);
    println!("Concatenated: {} | {}", s5, s6);

    // 5. Common String methods
    let mut s7 = String::from("abc");
    s7.push('d');
    s7.insert(1, 'X');
    s7.replace_range(2..3, "YZ");
    println!("Manipulated: {}", s7);
    let popped = s7.pop();
    println!("After pop: {}, popped = {:?}", s7, popped);
    let replaced = s7.replace("X", "*");
    println!("Replaced: {}", replaced);

    // 6. Splitting and iterating
    let s8 = "a,b,c";
    for part in s8.split(',') {
        print!("[{}] ", part);
    }
    println!("<- split");

    // 7. Unicode handling
    let s9 = String::from("你好，世界");
    println!("Unicode: {} | len = {} bytes", s9, s9.len());
    for c in s9.chars() {
        print!("{} ", c);
    }
    println!("<- chars");
    for b in s9.bytes() {
        print!("{} ", b);
    }
    println!("<- bytes");

    // 8. OsString/OsStr
    let oss: OsString = OsString::from("path/文件");
    let osstr: &OsStr = oss.as_os_str();
    println!("OsString: {:?}, OsStr: {:?}", oss, osstr);

    // 9. CString/CStr
    let cstring = CString::new("hello\0world").expect("CString::new failed");
    let cstr: &CStr = cstring.as_c_str();
    println!("CString: {:?}, CStr: {:?}", cstring, cstr);

    // 10. 'static lifetime for string literals
    // String literals like "hello" are of type &'static str
    let s_static: &'static str = "I am a static string literal";
    println!("s_static: {}", s_static);

    // 'static String: manually allocate a String with static lifetime (rare, advanced)
    // Usually via Box::leak or lazy_static/static once_cell
    let s_boxed: &'static String = Box::leak(Box::new(String::from("I am a static String")));
    println!("s_boxed: {}", s_boxed);
}
```

## Common String Operations

- Creating and converting between &str and String
- Concatenation (`+`, `format!`)
- Common methods: `push`, `push_str`, `pop`, `insert`, `replace`, `replace_range`, `split`, `chars`, `bytes`, `len`
- Unicode-safe iteration
- Platform-native strings: OsString/OsStr
- FFI (C API) strings: CString/CStr
- Static string literals: &'static str
- Static String allocation: &'static String (rare, advanced)

---

## Key Points

- Use `&str` for immutable, borrowed string data (string literals, slices).
- Use `String` for owned, growable string data.
- Use `OsString`/`OsStr` for file paths and OS APIs.
- Use `CString`/`CStr` for C FFI.
- Use `&'static str` for static string literals.
- Use `&'static String` for manually allocated static Strings (rare, advanced).
- All Rust strings are valid UTF-8 except OsString/OsStr and CString/CStr.

---

Run the code:

```sh
cargo run --bin ch09_06_string_types
