# 8.4 自定义错误与 `?` 运算符

本章演示如何在 Rust 中定义自定义错误类型，实现错误类型转换，并用 `?` 运算符简化错误传播。

## 示例代码

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO 错误: {}", e),
            MyError::Parse(e) => write!(f, "解析错误: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn read_and_parse_number(path: &str) -> Result<i32, MyError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_and_parse_number("number.txt") {
        Ok(n) => println!("读取到的数字: {}", n),
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

## 要点速览

- 通过自定义枚举统一多种错误类型。
- 实现 `Display` 和 `Error` trait 以获得更友好的错误信息和兼容性。
- 实现 `From` trait 以便 `?` 运算符自动完成错误类型转换。
- `?` 运算符遇到错误时会自动调用 `From` 进行转换。
- 该模式是 Rust 真实项目中健壮错误处理的惯用法。

---

运行代码：

```sh
cargo run --bin ch08_04_custom_error
```
