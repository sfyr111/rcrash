# Chapter 4.8: if let 语法糖

## Key Points

- `if let` 是 match 的简写，适用于只关心一种模式的情况。
- 常用于 Option、Result、枚举等类型的简洁匹配。
- 可以配合 else、while let 使用。

## Demo Code

```rust
fn main() {
    // 基本 if let 用法
    let opt = Some(10);
    if let Some(x) = opt {
        println!("Option contains {}", x);
    } else {
        println!("Option is None");
    }

    // 匹配枚举
    enum Status {
        Ok(u32),
        Error(String),
    }
    let s = Status::Ok(200);
    if let Status::Ok(code) = s {
        println!("Success: {}", code);
    } else {
        println!("Not Ok");
    }

    // while let 结合用法
    let mut opt = Some(3);
    while let Some(n) = opt {
        println!("n = {}", n);
        if n == 0 {
            opt = None;
        } else {
            opt = Some(n - 1);
        }
    }

    // if let else（Rust 1.65+）
    let opt = None;
    if let Some(x) = opt {
        println!("Found {}", x);
    } else {
        println!("Nothing found");
    }
}
```

## Table: if let 用法

| 用法                | 示例                               | 说明                                |
|---------------------|------------------------------------|-------------------------------------|
| Option 匹配         | `if let Some(x) = opt {}`          | 只关心 Some 的情况                  |
| 枚举匹配            | `if let Status::Ok(code) = s {}`   | 只关心某一枚举分支                  |
| 结合 else           | `if let ... else {}`               | 不匹配时执行 else                   |
| while let           | `while let Some(x) = opt {}`       | 循环解包 Option/枚举                |

## Notes

- `if let` 适合只关心一种情况，替代繁琐的 match。
- 多分支/复杂匹配仍建议用 match。
- Rust 1.65+ 支持 `if let ... else`。

---

> See also: [Rust Book: if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
