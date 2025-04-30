# 6.6 自动派生

本节演示 Rust 的 `#[derive]` 属性，自动为类型实现常用 trait。

## 示例：使用 `#[derive]`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: 3, y: 4 };
    let d = Point { ..Default::default() };

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d (default): {:?}", d);

    println!("a == b: {}", a == b);
    println!("a < c: {}", a < c);
    println!("Cloned c: {:?}", c.clone());
}
```

## 要点速览

- `#[derive]` 属性可自动为结构体或枚举实现 Debug、Clone、Copy、PartialEq、Eq、PartialOrd、Ord、Default 等常用 trait。
- 派生 trait 要求所有字段也实现该 trait。
- 自动派生减少样板代码，保证一致性。

## 对比表格

| trait         | 作用                         |
|-------------- |------------------------------|
| Debug         | 支持 `{:?}` 格式化输出        |
| Clone, Copy   | 值的复制                     |
| PartialEq, Eq | 相等性比较                   |
| PartialOrd, Ord | 顺序比较                   |
| Default       | 提供默认值                   |

## 补充说明

- 可在一个 `#[derive(...)]` 中组合多个 trait。
- 如需自定义行为，可手动实现 trait。
