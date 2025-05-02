# 6.1 泛型类型（Generic Type）

本章介绍 Rust 中的泛型类型，包括泛型的基本语法、应用场景和常见业务用法。

---

## 主要内容

- 泛型的定义与使用
- 函数、结构体、枚举中的泛型
- 泛型约束（trait bounds）
- 业务开发中的泛型优势

---

## 示例代码

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("最大值: {}", largest(&numbers));
}
```

---

## 要点

- 泛型让代码更灵活、可复用
- 常用于集合、算法、业务通用模型
- 约束泛型以保证类型安全

---

运行代码：

```sh
cargo run --bin ch06_01_generic_type
```
