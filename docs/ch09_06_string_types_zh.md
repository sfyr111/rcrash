# 9.6 Rust 中的多种字符串类型

本章详细讲解 Rust 常见字符串类型的用法、区别与最佳实践。

---

## 核心字符串类型

1. **&str**：字符串切片，对 UTF-8 字符串的不可变视图，常见于字面量或切片。
2. **String**：可增长、堆分配的 UTF-8 字符串。
3. **OsString/OsStr**：平台原生字符串（如文件路径等）。
4. **CString/CStr**：与 C API 交互用的字符串（以 null 结尾）。

---

## 示例代码

```rust
use std::ffi::{CString, CStr, OsString, OsStr};

fn main() {
    // 1. &str：字符串切片
    let s1: &str = "Hello, world!";
    println!("&str: {}", s1);

    // 2. String：拥有所有权、可增长的字符串
    let mut s2 = String::from("Hello");
    s2.push(',');
    s2.push_str(" world");
    println!("String: {}", s2);

    // 3. &str 和 String 互转
    let s3: String = s1.to_string();
    let s4: &str = &s2;
    println!("转换: s3 = {}, s4 = {}", s3, s4);

    // 4. 字符串拼接
    let s5 = s2.clone() + "!";
    let s6 = format!("{} {}", s1, s3);
    println!("拼接: {} | {}", s5, s6);

    // 5. 常用 String 方法
    let mut s7 = String::from("abc");
    s7.push('d');
    s7.insert(1, 'X');
    s7.replace_range(2..3, "YZ");
    println!("操作后: {}", s7);
    let popped = s7.pop();
    println!("pop 后: {}, popped = {:?}", s7, popped);
    let replaced = s7.replace("X", "*");
    println!("替换后: {}", replaced);

    // 6. 分割与遍历
    let s8 = "a,b,c";
    for part in s8.split(',') {
        print!("[{}] ", part);
    }
    println!("<- split");

    // 7. Unicode 处理
    let s9 = String::from("你好，世界");
    println!("Unicode: {} | 字节长度 = {}", s9, s9.len());
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

    // 10. 'static 生命周期的字符串字面量
    // 字符串字面量如 "hello" 类型为 &'static str
    let s_static: &'static str = "我是静态字符串字面量";
    println!("s_static: {}", s_static);

    // 'static String：手动分配静态生命周期的 String（极少用）
    // 通常用 Box::leak 或 lazy_static/static once_cell
    let s_boxed: &'static String = Box::leak(Box::new(String::from("我是静态 String")));
    println!("s_boxed: {}", s_boxed);
}
```

## 常用字符串操作

- &str 和 String 的创建与转换
- 拼接（`+`、`format!`）
- 常用方法：`push`、`push_str`、`pop`、`insert`、`replace`、`replace_range`、`split`、`chars`、`bytes`、`len`
- Unicode 安全遍历
- 平台原生字符串：OsString/OsStr
- FFI（C API）字符串：CString/CStr

---

## 要点速览

- 借用不可变字符串用 `&str`（字面量、切片）。
- 需要拥有所有权和可变字符串用 `String`。
- 操作文件路径等平台相关字符串用 `OsString`/`OsStr`。
- 与 C API 交互用 `CString`/`CStr`。
- Rust 字符串均为有效 UTF-8（OsString/OsStr、CString/CStr 除外）。

---

运行代码：

```sh
cargo run --bin ch09_06_string_types
