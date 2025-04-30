# 6.4 结构体泛型的实现

本节演示如何为泛型结构体实现方法，包括通用实现和特化实现（针对具体类型）。

## 示例：带方法的泛型结构体

```rust
/// 通用的二维点结构体，可存储任意类型。
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// 创建一个新的点。
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// 消耗点并返回元组。
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Point<f64> {
    /// 计算到原点的距离（仅限 f64）。
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    println!("p1: {:?}, tuple: {:?}", p1, p1.into_tuple());

    let p2 = Point::new(1.5, 2.5);
    println!("p2: {:?}, tuple: {:?}", p2, p2.into_tuple());

    let p3 = Point::new(0.0_f64, 5.0_f64);
    println!("p3: {:?}, distance from origin: {}", p3, p3.distance_from_origin());
}
```

## 要点速览

- 用 `impl<T> Struct<T>` 实现通用泛型方法。
- 用 `impl Struct<具体类型>` 实现特定类型的方法。
- 可以为同一个结构体同时实现泛型方法和特化方法。

## 对比表格

| 实现                         | 说明                        |
|------------------------------|-----------------------------|
| `impl<T> Point<T>`           | 所有类型 T 的通用方法       |
| `impl Point<f64>`            | 仅适用于 `Point<f64>` 的方法 |

## 补充说明

- 特化实现可为特定类型增加专属方法。
- 只有具体类型匹配时才能调用特化方法。
