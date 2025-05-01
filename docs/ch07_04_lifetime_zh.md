# 7.4 Rust 中的生命周期（Lifetime）

本章演示 Rust 的生命周期系统、生命周期标注，以及它如何防止悬垂引用。

## 示例代码

```rust
fn main() {
    // 基本生命周期用法：返回较长字符串的引用
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'.", result);

    // 悬垂引用示例（取消注释可见编译错误）
    // let reference_to_nothing = dangle();
}

// 生命周期标注：参数和返回值都必须至少活得和 'a 一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 此函数无法通过编译，因为它返回了局部变量的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 错误：`s` 生命周期不够长
// }
```

## 要点速览

- **生命周期**保证引用始终有效。
- 生命周期标注（`<'a>`）告诉编译器引用需要保持多长时间有效。
- 编译器在编译期阻止悬垂引用。
- 大多数生命周期可自动推断，复杂场景（如返回引用的函数）需显式标注。
- 返回局部变量引用是不允许的，会导致悬垂引用。

## 生命周期标注示例表格

| 函数签名                                 | 说明                       |
|------------------------------------------|----------------------------|
| `fn longest<'a>(x: &'a str, y: &'a str)` | 输入输出都共用生命周期 'a  |
| `fn dangle() -> &String`                 | 错误：返回局部变量引用     |

---

运行代码：

```sh
cargo run --bin ch07_04_lifetime
```
