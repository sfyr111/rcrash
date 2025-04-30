# 6.5 使用 Traits 定义共同的行为

本节演示如何在 Rust 中用 trait 定义不同类型的共同行为。

## 示例：Trait 及其实现

```rust
/// 可描述自身的 trait。
pub trait Describable {
    fn describe(&self) -> String;
}

/// 表示人的结构体。
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {} ({} years old)", self.name, self.age)
    }
}

/// 表示公司的结构体。
pub struct Company {
    pub name: String,
    pub employee_count: usize,
}

impl Describable for Company {
    fn describe(&self) -> String {
        format!("Company: {} ({} employees)", self.name, self.employee_count)
    }
}

/// 泛型函数，打印实现 Describable 的类型的描述。
pub fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let alice = Person { name: "Alice".to_string(), age: 30 };
    let acme = Company { name: "Acme Corp".to_string(), employee_count: 100 };

    print_description(&alice);
    print_description(&acme);
}
```

## 要点速览

- trait 定义类型可实现的共同行为。
- 用 `impl Trait for Type` 为类型实现 trait。
- trait bound（如 `T: Describable`）让泛型函数接受实现该 trait 的任意类型。

## 对比表格

| 类型      | 实现           | 输出示例                        |
|-----------|----------------|---------------------------------|
| Person    | Describable    | Person: Alice (30 years old)     |
| Company   | Describable    | Company: Acme Corp (100 employees)|

## 补充说明

- trait 是 Rust 多态和代码复用的核心。
- 可以为任意类型实现 trait，包括外部类型（用 newtype 模式）。
