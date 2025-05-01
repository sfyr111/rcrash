# 7.2 Rust 中的所有权（Ownership）

本节演示 Rust 所有权的核心概念，包括移动语义、克隆、函数调用中的所有权转移等。

## 示例：所有权基础

```rust
fn main() {
    // 所有权转移（move）
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被移动到 s2
    // println!("s1: {}", s1); // 错误：值已被移动
    println!("s2: {}", s2);

    // 作用域与所有权
    {
        let scoped = String::from("scoped");
        println!("scoped in inner scope: {}", scoped);
    } // scoped 在此处被释放
    // println!("scoped: {}", scoped); // 错误：未找到此变量

    // 克隆堆数据
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // 函数调用中的所有权转移（move）
    let s5 = String::from("Rust");
    takes_ownership(s5); // s5 被移动
    // println!("s5: {}", s5); // 错误：值已被移动

    // 函数调用中的引用（借用）
    let s6 = String::from("reference");
    borrows_ownership(&s6); // s6 被借用，未移动
    println!("s6 after borrow: {}", s6); // s6 依然有效

    let x = 5;
    makes_copy(x); // i32 实现了 Copy，x 依然有效
    println!("x: {}", x);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn borrows_ownership(s: &String) {
    println!("borrows_ownership: {}", s);
}

fn makes_copy(x: i32) {
    println!("makes_copy: {}", x);
}
```

## 要点速览

- Rust 中每个值有唯一所有者。
- 赋值或传参会移动所有权，原变量失效。
- 如 String 这类类型会 move，i32 等简单类型实现了 Copy，赋值后仍然有效。
- 用 `.clone()` 可显式深拷贝堆数据。
- 传递给函数时，所有权也会转移，除非类型实现了 Copy。
- 可以用引用（`&T`）借用值，让函数访问数据而不获取所有权。
- 变量离开作用域时会被自动释放（drop）。

## 对比表格

| 类型    | 赋值行为 | 函数传参 | clone    |
|---------|----------|----------|----------|
| String  | move     | move     | 深拷贝   |
| i32     | copy     | copy     | 拷贝     |

## 补充说明

- 所有权是 Rust 内存安全的基础。
- 使用已被移动的值会导致编译错误。
