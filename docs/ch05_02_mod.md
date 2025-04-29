# 5.2 模块化编程（Modular Programming in Rust）

## 关键要点

- Rust 的模块（module）通过 `mod` 关键字声明，用于组织代码、控制可见性和实现封装。
- 默认情况下，模块内的项是私有的（private），需要 `pub` 关键字公开。
- 模块可以嵌套，形成层级结构。
- 使用 `::` 语法访问模块中的公开项。
- 私有项只能在当前模块及其子模块中访问。

## 示例代码

```rust
// ch05_02_mod.rs
// Demonstration of modular programming in Rust
// Showcases module declaration, visibility, and usage.

mod math {
    // Private function (default)
    fn private_add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Public function
    pub fn add(a: i32, b: i32) -> i32 {
        private_add(a, b)
    }

    // Nested module
    pub mod advanced {
        pub fn square(x: i32) -> i32 {
            x * x
        }
    }
}

fn main() {
    // Using public function from module
    let sum = math::add(2, 3);
    println!("2 + 3 = {}", sum);

    // Using public function from nested module
    let squared = math::advanced::square(4);
    println!("4 squared = {}", squared);

    // The following line would not compile (private function):
    // let _ = math::private_add(2, 3);
}
```

## 表格：模块可见性对比

| 作用域        | 访问私有项 | 访问公有项 |
|---------------|:----------:|:----------:|
| 当前模块      | ✅         | ✅         |
| 子模块        | ✅         | ✅         |
| 父模块/外部   | ❌         | ✅         |

## 注意事项

- 推荐将相关功能组织进模块，提升代码可维护性和复用性。
- 使用 `pub mod` 和 `pub fn` 逐步暴露 API，隐藏实现细节。
- Rust 项目中可将模块拆分为多个文件，复杂项目建议使用 `mod.rs` 或文件夹结构。

---

> 参考：[Rust Book - 模块系统](https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html)
