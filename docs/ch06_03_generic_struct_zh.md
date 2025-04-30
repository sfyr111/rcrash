# 6.3 结构体中的泛型

本节演示如何在 Rust 结构体中使用泛型，编写灵活、可复用的数据结构。

## 示例：泛型结构体

```rust
/// 通用的二维点结构体，可存储任意类型。
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// 支持两个不同类型的 Pair 结构体。
#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

fn main() {
    let int_point = Point { x: 3, y: 4 };
    let float_point = Point { x: 1.2, y: 3.4 };
    let string_point = Point { x: "hello", y: "world" };

    println!("int_point: {:?}", int_point);
    println!("float_point: {:?}", float_point);
    println!("string_point: {:?}", string_point);

    let pair = Pair { first: 42, second: "answer" };
    println!("pair: {:?}", pair);
}
```

## 要点速览

- 使用 `<T>` 或 `<T, U>` 声明结构体的泛型参数。
- 泛型让结构体可以存储任意类型，提高复用性。
- 只要类型参数实现了相关 trait，泛型结构体也可自动派生 `Debug`、`Clone`、`Copy` 等 trait。

## 对比表格

| 结构体         | 说明                        |
|----------------|-----------------------------|
| `Point<T>`     | 二维点，两个字段类型相同     |
| `Pair<T, U>`   | 一对值，类型可不同           |

## 补充说明

- 泛型结构体在编译期会针对每种具体类型实例化。
- 结构体方法可继续使用泛型或限定具体类型。
