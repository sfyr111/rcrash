# Chapter 4.7: match Syntax

## Key Points

- `match` 是 Rust 的强大分支控制结构，可用于数值、枚举、模式匹配等多种场景。
- 支持多分支、区间、守卫条件（if）、绑定变量等。
- `match` 也是表达式，可以有返回值。

## Demo Code

```rust
fn main() {
    // 基本 match 语句
    let n = 2;
    match n {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=6 => println!("four to six"),
        _ => println!("something else"),
    }

    // 匹配枚举
    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Green;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // 匹配 Option
    let maybe = Some(42);
    match maybe {
        Some(x) if x > 40 => println!("Large: {}", x),
        Some(x) => println!("Small: {}", x),
        None => println!("None"),
    }

    // 解构结构体
    struct Person {
        name: String,
        age: u8,
    }
    let p = Person { name: "Alice".to_string(), age: 30 };
    match p {
        Person { name, age: 30 } => println!("{} is 30 years old", name),
        Person { name, age } => println!("{} is {} years old", name, age),
    }

    // 解构元组
    let point = (0, -2);
    match point {
        (0, y) => println!("On y axis at y = {}", y),
        (x, 0) => println!("On x axis at x = {}", x),
        (x, y) => println!("At ({}, {})", x, y),
    }

    // match 作为表达式
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("grade = {}", grade);
}
```

## Table: match 用法

| 用法           | 示例                        | 说明                       |
|----------------|-----------------------------|----------------------------|
| 单值匹配       | `match n { 1 => ... }`      | 匹配单个值                 |
| 多值/或        | `2 | 3 => ...`              | 匹配多个值                 |
| 区间           | `4..=6 => ...`              | 匹配范围                   |
| 枚举           | `Color::Red => ...`         | 匹配枚举类型               |
| Option/Some    | `Some(x) => ...`            | 解包 Option/Result/自定义枚举 |
| 守卫条件       | `Some(x) if x > 40 => ...`  | 分支增加 if 条件           |
| 解构结构体     | `Person { name, age }`      | 字段解构                   |
| 解构元组       | `(x, y)`                    | 位置解构                   |
| 通配符         | `_ => ...`                  | 匹配其它所有情况           |
| 表达式         | `let x = match ... {}`      | match 有返回值             |

## Notes

- 所有分支必须覆盖所有可能性（穷尽性），通常用 `_` 兜底。
- 支持嵌套模式和复杂结构体解构。
- `match` 是 Rust 安全和强类型的重要体现。

---

> See also: [Rust Book: match 控制流](https://doc.rust-lang.org/book/ch06-02-match.html)
