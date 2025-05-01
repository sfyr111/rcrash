# 7.3 Rust 中的借用（Borrowing）

本章演示 Rust 的借用机制，包括不可变引用和可变引用，以及这些规则如何保证内存安全。

## 示例代码

```rust
fn main() {
    // 不可变借用
    let s = String::from("hello");
    let len = calculate_length(&s); // &s 是不可变引用
    println!("The length of '{}' is {}.", s, len); // s 依然有效

    // 允许多个不可变借用
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // 可变借用
    let mut s2 = String::from("world");
    change(&mut s2); // &mut s2 是可变引用
    println!("After change: {}", s2);

    // 同一时刻只能有一个可变借用
    // let r3 = &mut s2;
    // let r4 = &mut s2; // 错误：同一时刻不能有多个可变引用

    // 不可变引用与可变引用不能同时存在
    // let r5 = &s2;
    // let r6 = &mut s2; // 错误：已存在不可变引用时不能再借用为可变引用
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", borrow!");
}
```

## 要点速览

- **不可变引用**（`&T`）允许只读访问数据，不获取所有权。
- **多个不可变引用**可以同时存在。
- **可变引用**（`&mut T`）允许修改数据，但同一时刻只能有一个可变引用。
- **不可变引用和可变引用不能同时存在于同一作用域。**
- 这些规则在编译期防止数据竞争。

## 借用规则对比表

| 引用类型           | 是否允许多个 | 是否可变 | 能否与另一种共存 |
|--------------------|--------------|----------|------------------|
| 不可变（`&T`）     | 是           | 否       | 不能与可变共存   |
| 可变（`&mut T`）   | 否           | 是       | 不能与不可变共存 |

---

运行代码：

```sh
cargo run --bin ch07_03_borrowing
```
