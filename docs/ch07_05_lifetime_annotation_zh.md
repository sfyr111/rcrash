# 7.5 生命周期注解（Lifetime Annotations）

本章演示函数、结构体和方法中的生命周期注解，并解释何时以及为何需要显式标注。

## 示例代码

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn summary(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }

    // 返回引用时需要生命周期注解，保证与 self 生命周期一致
    fn title(&self) -> &'a str {
        self.title
    }
}

// 多个引用参数时需显式生命周期注解
fn longest_with_announcement<'a>(x: &'a str, y: &'a str, ann: &str) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let b = Book { title: "Rust Book", author: "Steve" };
    println!("Book summary: {}", b.summary());
    println!("Book title: {}", b.title());

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = longest_with_announcement(s1.as_str(), s2.as_str(), "Comparing greetings");
    println!("Longest: {}", result);
}
```

## 要点速览

- 生命周期注解（如 `<'a>`）用于关联引用的生命周期，保证引用始终有效。
- 持有引用的结构体必须显式声明生命周期参数。
- 方法返回引用时，通常需要生命周期注解以保证返回值生命周期。
- 多个引用参数的函数，如关系无法自动推断，也需显式生命周期。
- 生命周期注解**不会**改变数据的实际生命周期，只描述引用之间的关系。

## 生命周期注解常见场景表

| 场景                     | 是否需要生命周期注解 | 示例                                 |
|--------------------------|----------------------|--------------------------------------|
| 单一引用参数的函数       | 否（可省略）         | `fn foo(x: &str)`                    |
| 多引用参数/返回引用的函数 | 是                   | `fn bar<'a>(x: &'a str, y: &'a str) -> &'a str` |
| 持有引用的结构体         | 是                   | `struct Foo<'a> { x: &'a str }`      |
| 返回引用的方法           | 通常需要             | `fn get(&self) -> &str`              |

---

运行代码：

```sh
cargo run --bin ch07_05_lifetime_annotation
```
