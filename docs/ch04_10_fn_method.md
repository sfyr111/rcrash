# Chapter 4.10: 函数与方法

## Key Points

- Rust 使用 `fn` 关键字定义函数。
- 函数有参数和返回值，类型必须显式声明。
- 方法（method）定义在 `impl` 块内，第一个参数通常为 `self`，用于操作结构体实例。
- 关联函数（associated function）无 `self` 参数，类似静态方法。

## Demo Code

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_greeting(name: &str) {
    println!("Hello, {}!", name);
}

struct Counter {
    value: i32,
}

impl Counter {
    // 关联函数（静态方法）
    fn new(start: i32) -> Self {
        Counter { value: start }
    }
    // 方法（有 self 参数）
    fn inc(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    // 普通函数
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    print_greeting("Rustacean");

    // 方法与关联函数
    let mut c = Counter::new(10);
    c.inc();
    c.inc();
    println!("Counter value = {}", c.get());
}
```

## Table: 函数与方法用法

| 用法         | 示例                           | 说明                     |
|--------------|--------------------------------|--------------------------|
| 普通函数     | `fn add(a: i32, b: i32)`      | 定义自由函数             |
| 返回值       | `-> i32`                      | 明确类型，`return` 可省略 |
| 关联函数     | `fn new(...) -> Self`         | `impl` 内、无 self 参数   |
| 方法         | `fn inc(&mut self)`           | `impl` 内、有 self 参数   |
| 调用方法     | `c.inc()`                     | 用实例调用方法           |
| 调用关联函数 | `Counter::new(10)`            | 类型名调用，无实例       |

## Notes

- Rust 不支持函数重载。
- 方法必须定义在 `impl` 块内。
- `&self` 表示只读借用，`&mut self` 表示可变借用。
- 关联函数常用于构造器（如 `new`）。

---

> See also: [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
> See also: [Rust Book: Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
