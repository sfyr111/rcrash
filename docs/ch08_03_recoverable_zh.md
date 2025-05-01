# 8.3 可恢复的错误（Recoverable Errors）

本章演示如何使用 `Result` 类型、错误传播和常见 Rust 错误处理习惯来应对可恢复错误。

## 示例代码

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 示例 1：打开文件（可能失败）
    match File::open("hello.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or(0);
            println!("文件打开成功: {}", contents);
        }
        Err(e) => {
            println!("打开文件失败: {}", e);
        }
    }

    // 示例 2：用 Result 传播错误
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名出错: {}", e),
    }

    // 示例 3：unwrap 和 expect（生产环境不推荐）
    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 用 Result 传播错误的函数
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

## 🏆 实战：通用文件读取与错误提示

```rust
use std::fs::File;
use std::io::{self, Read};

/// 通用函数：读取文件内容，自动传播错误
fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename = "hello.txt";
    match read_file_content(filename) {
        Ok(text) => println!("文件内容：\n{}", text),
        Err(e) => eprintln!("无法读取文件 [{}]，原因：{}", filename, e),
    }
}
```

- `read_file_content` 使用 `?` 运算符自动传播错误，代码简洁。
- `main` 函数分离业务逻辑与错误处理，`eprintln!` 用于输出错误。

## 要点速览

- 对可能失败的操作使用 `Result<T, E>` 类型。
- 使用模式匹配（`match`）分别处理 Ok 和 Err。
- `?` 操作符可自动传播错误，遇错提前返回。
- `unwrap()` 和 `expect()` 仅适合原型开发或确信不会失败的场景。
- 可根据需要自定义错误处理逻辑。

## 可恢复错误常见写法表

| 写法            | 示例代码                                          | 说明                         |
|-----------------|---------------------------------------------------|------------------------------|
| 模式匹配        | `match File::open("file") { ... }`               | 显式处理 Ok/Err              |
| `?` 传播错误    | `let f = File::open("file")?;`                   | 有错直接返回给调用者         |
| unwrap/expect   | `file.read_to_string(&mut s).unwrap()`            | 出错 panic（生产环境不推荐）  |

---

运行代码：

```sh
cargo run --bin ch08_03_recoverable
