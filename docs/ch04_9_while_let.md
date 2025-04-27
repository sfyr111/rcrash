# Chapter 4.9: while let 语法糖

## Key Points

- `while let` 是 match/if let 的简写，适用于循环中持续解包和匹配。
- 常用于 Option、Result、枚举等类型的循环处理。
- 可以结合模式守卫（if 条件）使用。

## Demo Code

```rust
fn main() {
    // 基本用法：Option
    let mut opt = Some(5);
    while let Some(n) = opt {
        println!("n = {}", n);
        if n == 0 {
            opt = None;
        } else {
            opt = Some(n - 1);
        }
    }

    // 结合 Result
    let mut results = vec![Ok(1), Ok(2), Err("fail")].into_iter();
    while let Some(Ok(x)) = results.next() {
        println!("Ok: {}", x);
    }

    // 模式守卫
    let mut opt = Some(3);
    while let Some(n) = opt {
        if n % 2 == 0 {
            println!("even: {}", n);
            break;
        }
        println!("odd: {}", n);
        opt = if n > 0 { Some(n - 1) } else { None };
    }
}
```

## Table: while let 用法

| 用法             | 示例                                 | 说明                              |
|------------------|--------------------------------------|-----------------------------------|
| Option 匹配      | `while let Some(x) = opt {}`         | 持续解包 Option                   |
| Result 匹配      | `while let Some(Ok(x)) = iter.next()`| 结合迭代器和 Result               |
| 模式守卫         | `while let Some(n) = opt { if ... }` | 分支内可加条件判断                |

## Notes

- `while let` 适合循环中只关心一种匹配情况。
- 多分支或复杂循环建议用 match 或普通 while。
- 可用于 Option、Result、自定义枚举等。

---

> See also: [Rust Book: while let](https://doc.rust-lang.org/book/ch06-03-if-let.html#while-let-conditional-loops)
