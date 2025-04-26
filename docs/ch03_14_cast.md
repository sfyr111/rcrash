# Chapter 3.14: Type Conversion in Rust

## Key Points

- Rust requires explicit type conversion (casting) between most types.
- Use `as` for primitive type casts (e.g., integer to float, char to u32).
- Use `.parse()` and `.to_string()` for string/number conversions.
- `to_string()` and `as_str()`/`as_deref()` are common for string slices and String.
- Some conversions may fail and return a `Result` (e.g., `parse()`), so handle errors appropriately.
- **std::convert** provides `From`, `Into`, `TryFrom`, `TryInto` traits for safe and idiomatic conversions.
- **std::mem::transmute** allows for unchecked, dangerous, low-level conversions (only for advanced use cases).

## Demo Code

```rust
fn main() {
    // Integer to float
    let i = 42;
    let f = i as f64;
    println!("i = {}, f = {}", i, f);

    // Float to integer (truncates toward zero)
    let pi = 3.1415;
    let n = pi as i32;
    println!("pi = {}, n = {}", pi, n);

    // Integer to char (Unicode scalar value)
    let c = 65u8 as char;
    println!("65 as char: {}", c);

    // Char to u32 (Unicode code point)
    let code = 'A' as u32;
    println!("'A' as u32: {}", code);

    // String to integer (parse)
    let s = "123";
    let num: i32 = s.parse().unwrap();
    println!("'{}' parsed to i32: {}", s, num);

    // Integer to string (to_string)
    let n = 456;
    let s = n.to_string();
    println!("{} to string: '{}'", n, s);

    // &str to String and back
    let s1 = "hello";
    let s2 = s1.to_string();
    let s3 = &s2[..];
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    // Option: using as_ref and as_deref
    let s = Some(String::from("world"));
    let s_ref: Option<&str> = s.as_deref();
    println!("Option<String> as_deref: {:?}", s_ref);

    // --- std::convert: From, Into, TryFrom, TryInto ---
    use std::convert::{From, Into, TryFrom, TryInto};

    let s: String = String::from("convert");
    let s2: String = "convert2".into(); // &str -> String
    println!("From/Into: s = {}, s2 = {}", s, s2);

    let n: i32 = i32::from(10u8); // u8 -> i32
    println!("i32::from(10u8): {}", n);

    let res: Result<i32, _> = i32::try_from(100u8); // always Ok
    println!("i32::try_from(100u8): {:?}", res);

    let val: Result<u8, _> = 300i32.try_into(); // will Err (overflow)
    println!("300i32.try_into::<u8>(): {:?}", val);

    // --- std::mem::transmute (DANGEROUS!) ---
    use std::mem::transmute;
    let a: u32 = 0x61626364;
    let b: [u8; 4] = unsafe { transmute(a) };
    println!("transmute u32 to [u8;4]: {:?}", b); // platform-dependent
}
```

## Table: Common Type Conversions

| Conversion                | Example                          | Notes                              |
|---------------------------|----------------------------------|------------------------------------|
| int → float               | `let f = i as f64;`              | Explicit cast                      |
| float → int               | `let n = pi as i32;`             | Truncates toward zero              |
| int → char                | `let c = 65u8 as char;`          | Unicode scalar value               |
| char → u32                | `let code = 'A' as u32;`         | Unicode code point                 |
| str → int                 | `let n: i32 = s.parse()?;`       | Returns Result, may fail           |
| int → String              | `let s = n.to_string();`         |                                    |
| &str → String             | `let s2 = s1.to_string();`        |                                    |
| String → &str             | `let s3 = &s2[..];`              |                                    |
| Option<String> → Option<&str> | `as_deref()`                  | Converts Option<String> to Option<&str> |
| &str → String (From/Into) | `let s: String = s1.into();`     | Idiomatic, safe                    |
| u8 → i32 (From)           | `let n = i32::from(10u8);`       |                                    |
| i32 → u8 (TryInto)        | `let val: Result<u8, _> = n.try_into();` | May fail (overflow)      |
| Unsafe memory cast        | `unsafe { transmute(x) }`        | DANGEROUS, only for experts        |

## Notes

- Rust does not do implicit numeric or string conversions.
- Use pattern matching or `unwrap_or` to handle failed parses.
- More conversions are available via the `From` and `Into` traits for custom types.
- **Never use `transmute` unless you are 100% sure of what you are doing!**

---

> For more details, see [Rust Book: Converting Types](https://doc.rust-lang.org/book/ch03-05-control-flow.html#converting-between-types) and [std::convert docs](https://doc.rust-lang.org/std/convert/index.html)
