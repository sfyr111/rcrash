# 6.2 泛型作为函数参数的类型

本节演示如何在 Rust 中使用泛型作为函数参数类型，写出灵活且可复用的代码。

## 示例：泛型函数

```rust
/// 返回两个值中较大的一个。
/// 适用于实现了 PartialOrd 和 Copy trait 的任意类型。
pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// 交换元组中的两个值。
pub fn swap<T>(pair: (T, T)) -> (T, T) {
    let (x, y) = pair;
    (y, x)
}

fn main() {
    let a = 10;
    let b = 20;
    println!("max({}, {}) = {}", a, b, max(a, b));

    let x = 3.14;
    let y = 2.71;
    println!("max({}, {}) = {}", x, y, max(x, y));

    let pair = ("hello", "world");
    let swapped = swap(pair);
    println!("swap({:?}) = {:?}", pair, swapped);
}
```

## 要点速览

- 使用 `<T>` 声明函数的泛型类型参数。
- Trait 约束（如 `T: PartialOrd + Copy`）限制可用类型。
- 泛型提升了代码复用性和类型安全。

## 对比表格

| 函数      | 说明                        |
|-----------|-----------------------------|
| `max<T>`  | 返回两个 `T` 类型值中的较大者 |
| `swap<T>` | 交换元组中两个 `T` 类型的值   |

## 补充说明

- 泛型函数在编译期针对每种具体类型单独生成代码（单态化）。
- Rust 要求对比较、复制等操作显式声明 trait 约束。
