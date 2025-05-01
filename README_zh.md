# Rust 零基础速成示例集

本项目包含 Rust 编程课程的代码演示和配套 Markdown 讲解，适合自学、教学和快速查阅。

---

## 项目结构

```markdown
rcrash/
├── src/
│   └── bin/
│       ├── ch03_3_var_mut.rs           # 3.3 变量与可变性
│       ├── ch03_4_basic_types.rs       # 3.4 基本数据类型
│       ├── ch03_6_integer_overflow.rs  # 3.6 整数溢出
│       ├── ch03_7_tuple.rs             # 3.7 元组
│       ├── ch03_8_array.rs             # 3.8 数组
│       ├── ch03_9_slice.rs             # 3.9 切片类型
│       ├── ch03_10_struct.rs           # 3.10 结构体
│       ├── ch03_11_enum.rs             # 3.11 枚举
│       ├── ch03_12_comment.rs          # 3.12 注释类型
│       ├── ch03_13_println.rs          # 3.13 println! 宏
│       ├── ch03_14_cast.rs             # 3.14 类型转换
│       ├── ch04_2_expr.rs              # 4.2 表达式形式
│       ├── ch04_3_if.rs                # 4.3 if/else 选择结构
│       ├── ch04_4_loop.rs              # 4.4 loop 循环
│       ├── ch04_5_while.rs             # 4.5 while 循环
│       ├── ch04_6_for.rs               # 4.6 for/range 迭代
│       ├── ch04_7_match.rs             # 4.7 match 语法
│       ├── ch04_8_if_let.rs            # 4.8 if let 语法糖
│       ├── ch04_9_while_let.rs         # 4.9 while let 语法糖
│       ├── ch04_10_fn_method.rs        # 4.10 函数与方法
│       ├── ch04_11_fn_closure.rs       # 4.11 函数与闭包
│       ├── ch04_13_high_order.rs       # 4.13 高阶函数
│       ├── ch04_14_never.rs            # 4.14 发散函数（Never 类型）
│       ├── ch04_15_guess.rs            # 4.15 猜数字游戏
│       ├── ch05_02_mod.rs               # 5.2 Rust 模块化编程
│       ├── ch05_03_pub.rs               # 5.3 Rust 可见性控制
│       ├── ch05_04_struct_visibility.rs # 5.4 Rust 结构体可见性
│       ├── ch05_05_use_binding.rs       # 5.5 使用 use 绑定模块成员
│       ├── ch05_06_super_self.rs        # 5.6 使用 super 与 self 简化路径
│       ├── ch05_08_project_structure.md # 5.8 Rust 项目目录层次结构（文档）
│       ├── ch07_02_ownership.rs         # 7.2 Rust 中的所有权（Ownership）
│       ├── ch07_03_borrowing.rs         # 7.3 Rust 中的借用（Borrowing）
│       ├── ch07_04_lifetime.rs          # 7.4 Rust 中的生命周期（Lifetime）
│       ├── ch07_05_lifetime_annotation.rs # 7.5 生命周期注解（Lifetime Annotations）
│       ├── ch08_02_unrecoverable.rs     # 8.2 不可恢复的错误
│       ├── ch08_03_recoverable.rs     # 8.3 可恢复的错误
│       ├── ch08_04_custom_error.rs     # 8.4 自定义错误与 ? 运算符
│       └── ...                         # 更多章节示例
├── docs/
│   ├── ch03_3_var_mut.md               # 3.3 变量与可变性
│   ├── ch03_4_basic_types.md           # 3.4 基本数据类型
│   ├── ch03_6_integer_overflow.md      # 3.6 整数溢出
│   ├── ch03_7_tuple.md                 # 3.7 元组
│   ├── ch03_8_array.md                 # 3.8 数组
│   ├── ch03_9_slice.md                 # 3.9 切片类型
│   ├── ch03_10_struct.md               # 3.10 结构体
│   ├── ch03_11_enum.md                 # 3.11 枚举
│   ├── ch03_12_comment.md              # 3.12 注释类型
│   ├── ch03_13_println.md              # 3.13 println! 宏
│   ├── ch03_14_cast.md                 # 3.14 类型转换
│   ├── ch04_2_expr.md                  # 4.2 表达式形式
│   ├── ch04_3_if.md                    # 4.3 if/else 选择结构
│   ├── ch04_4_loop.md                  # 4.4 loop 循环
│   ├── ch04_5_while.md                 # 4.5 while 循环
│   ├── ch04_6_for.md                   # 4.6 for/range 迭代
│   ├── ch04_7_match.md                 # 4.7 match 语法
│   ├── ch04_8_if_let.md                # 4.8 if let 语法糖
│   ├── ch04_9_while_let.md             # 4.9 while let 语法糖
│   ├── ch04_10_fn_method.md            # 4.10 函数与方法
│   ├── ch04_11_fn_closure.md           # 4.11 函数与闭包
│   ├── ch04_13_high_order.md           # 4.13 高阶函数
│   ├── ch04_14_never.md                # 4.14 发散函数（Never 类型）
│   ├── ch04_15_guess.md                # 4.15 猜数字游戏
│   ├── ch05_02_mod.md                   # 5.2 Rust 模块化编程
│   ├── ch05_03_pub.md                   # 5.3 Rust 可见性控制
│   ├── ch05_04_struct_visibility.md    # 5.4 Rust 结构体可见性
│   ├── ch05_05_use_binding.md          # 5.5 使用 use 绑定模块成员
│   ├── ch05_06_super_self.md           # 5.6 使用 super 与 self 简化路径
│   ├── ch05_08_project_structure.md    # 5.8 Rust 项目目录层次结构
│   ├── ch07_02_ownership.md            # 7.2 Rust 中的所有权（Ownership）
│   ├── ch07_03_borrowing.md            # 7.3 Rust 中的借用（Borrowing）
│   ├── ch07_04_lifetime.md             # 7.4 Rust 中的生命周期（Lifetime）
│   ├── ch07_05_lifetime_annotation.md # 7.5 生命周期注解（Lifetime Annotations）
│   ├── ch08_02_unrecoverable.md       # 8.2 不可恢复的错误
│   ├── ch08_03_recoverable.md       # 8.3 可恢复的错误
│   ├── ch08_04_custom_error.md       # 8.4 自定义错误与 ? 运算符
│   └── ...                             # 更多章节文档
└── README.md                           # 项目简介与索引
```

---

## 快速开始

1. 安装 Rust 工具链：[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. 克隆或下载本仓库
3. 运行任意章节示例，例如：

```bash
cargo run --bin ch03_3_var_mut
cargo run --bin ch03_4_basic_types
cargo run --bin ch03_6_integer_overflow
cargo run --bin ch03_7_tuple
cargo run --bin ch03_8_array
cargo run --bin ch03_9_slice
cargo run --bin ch03_10_struct
cargo run --bin ch03_11_enum
cargo run --bin ch03_12_comment
cargo run --bin ch03_13_println
cargo run --bin ch03_14_cast
cargo run --bin ch04_2_expr
cargo run --bin ch04_3_if
cargo run --bin ch04_4_loop
cargo run --bin ch04_5_while
cargo run --bin ch04_6_for
cargo run --bin ch04_7_match
cargo run --bin ch04_8_if_let
cargo run --bin ch04_9_while_let
cargo run --bin ch04_10_fn_method
cargo run --bin ch04_11_fn_closure
cargo run --bin ch04_13_high_order
cargo run --bin ch04_14_never
cargo run --bin ch04_15_guess
cargo run --bin ch05_02_mod
cargo run --bin ch05_03_pub
cargo run --bin ch05_04_struct_visibility
cargo run --bin ch05_05_use_binding
cargo run --bin ch05_06_super_self
cargo run --bin ch06_02_generic_fn_param
cargo run --bin ch06_03_generic_struct
cargo run --bin ch06_04_generic_struct_impl
cargo run --bin ch06_05_traits
cargo run --bin ch06_06_derive
cargo run --bin ch07_02_ownership
cargo run --bin ch07_03_borrowing
cargo run --bin ch07_04_lifetime
cargo run --bin ch07_05_lifetime_annotation
cargo run --bin ch08_02_unrecoverable
cargo run --bin ch08_03_recoverable
cargo run --bin ch08_04_custom_error
```

---

## 章节索引

--------------------------------------------------------

| 序号 | 主题                         | 示例代码                                                        | 配套文档                                                      |
|------|------------------------------|-----------------------------------------------------------------|---------------------------------------------------------------|
| 3.3  | 变量与可变性                 | [ch03_3_var_mut.rs](src/bin/ch03_3_var_mut.rs)                  | [ch03_3_var_mut.md](docs/ch03_3_var_mut.md)                   |
| 3.4  | 基本数据类型                 | [ch03_4_basic_types.rs](src/bin/ch03_4_basic_types.rs)          | [ch03_4_basic_types.md](docs/ch03_4_basic_types.md)            |
| 3.6  | 整数溢出                     | [ch03_6_integer_overflow.rs](src/bin/ch03_6_integer_overflow.rs)| [ch03_6_integer_overflow.md](docs/ch03_6_integer_overflow.md)  |
| 3.7  | 元组                         | [ch03_7_tuple.rs](src/bin/ch03_7_tuple.rs)                      | [ch03_7_tuple.md](docs/ch03_7_tuple.md)                        |
| 3.8  | 数组                         | [ch03_8_array.rs](src/bin/ch03_8_array.rs)                      | [ch03_8_array.md](docs/ch03_8_array.md)                        |
| 3.9  | 切片类型                     | [ch03_9_slice.rs](src/bin/ch03_9_slice.rs)                      | [ch03_9_slice.md](docs/ch03_9_slice.md)                        |
| 3.10 | 结构体                       | [ch03_10_struct.rs](src/bin/ch03_10_struct.rs)                  | [ch03_10_struct.md](docs/ch03_10_struct.md)                    |
| 3.11 | 枚举                         | [ch03_11_enum.rs](src/bin/ch03_11_enum.rs)                      | [ch03_11_enum.md](docs/ch03_11_enum.md)                        |
| 3.12 | 注释类型                     | [ch03_12_comment.rs](src/bin/ch03_12_comment.rs)                | [ch03_12_comment.md](docs/ch03_12_comment.md)                  |
| 3.13 | println! 宏                  | [ch03_13_println.rs](src/bin/ch03_13_println.rs)                | [ch03_13_println.md](docs/ch03_13_println.md)                  |
| 3.14 | 类型转换                     | [ch03_14_cast.rs](src/bin/ch03_14_cast.rs)                      | [ch03_14_cast.md](docs/ch03_14_cast.md)                        |
| 4.2  | 表达式形式                   | [ch04_2_expr.rs](src/bin/ch04_2_expr.rs)                        | [ch04_2_expr.md](docs/ch04_2_expr.md)                          |
| 4.3  | if/else 选择结构             | [ch04_3_if.rs](src/bin/ch04_3_if.rs)                            | [ch04_3_if.md](docs/ch04_3_if.md)                              |
| 4.4  | loop 循环                    | [ch04_4_loop.rs](src/bin/ch04_4_loop.rs)                        | [ch04_4_loop.md](docs/ch04_4_loop.md)                          |
| 4.5  | while 循环                   | [ch04_5_while.rs](src/bin/ch04_5_while.rs)                      | [ch04_5_while.md](docs/ch04_5_while.md)                        |
| 4.6  | for/range 迭代               | [ch04_6_for.rs](src/bin/ch04_6_for.rs)                          | [ch04_6_for.md](docs/ch04_6_for.md)                            |
| 4.7  | match 语法                   | [ch04_7_match.rs](src/bin/ch04_7_match.rs)                      | [ch04_7_match.md](docs/ch04_7_match.md)                        |
| 4.8  | if let 语法糖                | [ch04_8_if_let.rs](src/bin/ch04_8_if_let.rs)                    | [ch04_8_if_let.md](docs/ch04_8_if_let.md)                      |
| 4.9  | while let 语法糖             | [ch04_9_while_let.rs](src/bin/ch04_9_while_let.rs)              | [ch04_9_while_let.md](docs/ch04_9_while_let.md)                |
| 4.10 | 函数与方法                   | [ch04_10_fn_method.rs](src/bin/ch04_10_fn_method.rs)             | [ch04_10_fn_method.md](docs/ch04_10_fn_method.md)               |
| 4.11 | 函数与闭包                   | [ch04_11_fn_closure.rs](src/bin/ch04_11_fn_closure.rs)           | [ch04_11_fn_closure.md](docs/ch04_11_fn_closure.md)             |
| 4.13 | 高阶函数                     | [ch04_13_high_order.rs](src/bin/ch04_13_high_order.rs)           | [ch04_13_high_order.md](docs/ch04_13_high_order.md)             |
| 4.14 | 发散函数（Never 类型）        | [ch04_14_never.rs](src/bin/ch04_14_never.rs)                     | [ch04_14_never.md](docs/ch04_14_never.md)                      |
| 4.15 | 猜数字游戏                   | [ch04_15_guess.rs](src/bin/ch04_15_guess.rs)                     | [ch04_15_guess.md](docs/ch04_15_guess.md)                      |
| 5.2  | Rust 模块化编程              | [ch05_02_mod.rs](src/bin/ch05_02_mod.rs)                         | [ch05_02_mod.md](docs/ch05_02_mod.md)                          |
| 5.3  | Rust 可见性控制              | [ch05_03_pub.rs](src/bin/ch05_03_pub.rs)                         | [ch05_03_pub.md](docs/ch05_03_pub.md)                          |
| 5.4  | Rust 结构体可见性            | [ch05_04_struct_visibility.rs](src/bin/ch05_04_struct_visibility.rs) | [ch05_04_struct_visibility.md](docs/ch05_04_struct_visibility.md) |
| 5.5  | 使用 use 绑定模块成员         | [ch05_05_use_binding.rs](src/bin/ch05_05_use_binding.rs)          | [ch05_05_use_binding.md](docs/ch05_05_use_binding.md)           |
| 5.6  | 使用 super 与 self 简化路径   | [ch05_06_super_self.rs](src/bin/ch05_06_super_self.rs)            | [ch05_06_super_self.md](docs/ch05_06_super_self.md)             |
| 5.8  | Rust 项目目录层次结构         | — | [ch05_08_project_structure.md](docs/ch05_08_project_structure.md) |
| 6.2  | 泛型作为函数参数的类型         | [ch06_02_generic_fn_param.rs](src/bin/ch06_02_generic_fn_param.rs) | [ch06_02_generic_fn_param.md](docs/ch06_02_generic_fn_param.md) |
| 6.3  | 结构体中的泛型                   | [ch06_03_generic_struct.rs](src/bin/ch06_03_generic_struct.rs) | [ch06_03_generic_struct.md](docs/ch06_03_generic_struct.md) |
| 6.4  | 结构体中的泛型实现              | [ch06_04_generic_struct_impl.rs](src/bin/ch06_04_generic_struct_impl.rs) | [ch06_04_generic_struct_impl.md](docs/ch06_04_generic_struct_impl.md) |
| 6.5  | 使用 Traits 定义共同的行为         | [ch06_05_traits.rs](src/bin/ch06_05_traits.rs) | [ch06_05_traits.md](docs/ch06_05_traits.md) |
| 6.6  | 自动派生                         | [ch06_06_derive.rs](src/bin/ch06_06_derive.rs) | [ch06_06_derive.md](docs/ch06_06_derive.md) |
| 7.2  | Rust 中的所有权（Ownership）      | [ch07_02_ownership.rs](src/bin/ch07_02_ownership.rs) | [ch07_02_ownership.md](docs/ch07_02_ownership.md) |
| 7.3  | Rust 中的借用（Borrowing）        | [ch07_03_borrowing.rs](src/bin/ch07_03_borrowing.rs) | [ch07_03_borrowing.md](docs/ch07_03_borrowing.md) |
| 7.4  | Rust 中的生命周期（Lifetime）     | [ch07_04_lifetime.rs](src/bin/ch07_04_lifetime.rs) | [ch07_04_lifetime.md](docs/ch07_04_lifetime.md) |
| 7.5  | 生命周期注解（Lifetime Annotations） | [ch07_05_lifetime_annotation.rs](src/bin/ch07_05_lifetime_annotation.rs) | [ch07_05_lifetime_annotation.md](docs/ch07_05_lifetime_annotation.md) |
| 8.2  | 不可恢复的错误（Unrecoverable Errors） | [ch08_02_unrecoverable.rs](src/bin/ch08_02_unrecoverable.rs) | [ch08_02_unrecoverable.md](docs/ch08_02_unrecoverable.md) |
| 8.3  | 可恢复的错误（Recoverable Errors） | [ch08_03_recoverable.rs](src/bin/ch08_03_recoverable.rs) | [ch08_03_recoverable.md](docs/ch08_03_recoverable.md) |
| 8.4  | 自定义错误与 ? 运算符 | [ch08_04_custom_error.rs](src/bin/ch08_04_custom_error.rs) | [ch08_04_custom_error.md](docs/ch08_04_custom_error.md) |
| ...  | ...                          | ...                                                               | ...                                                               |

> 更多章节持续更新中...

---

## 说明

- 每一章的代码和文档都双向链接，便于快速查阅。
- 推荐结合 [Rust 官方书籍](https://kaisery.github.io/trpl-zh-cn/) 学习。
- 枚举的打印全部为手动 match，无自动 Debug。
- [ch05_02_mod.rs](src/bin/ch05_02_mod.rs)：模块化编程示例，演示如何声明模块、控制可见性、嵌套模块与使用 `pub` 公开 API。
- [ch05_03_pub.rs](src/bin/ch05_03_pub.rs)：可见性控制示例，包含 `pub(self)`、`pub(super)`、`pub(crate)`、`pub` 的用法与典型场景。
- [docs/ch05_02_mod.md](docs/ch05_02_mod.md)：模块化编程英文文档，包含模块声明、可见性说明、示例代码与对比表。
- [docs/ch05_03_pub.md](docs/ch05_03_pub.md)：可见性控制英文文档，详细解释 Rust 各种可见性修饰符及用法。
- [docs/ch05_04_struct_visibility.md](docs/ch05_04_struct_visibility.md)：结构体可见性英文文档，详细说明 struct 及字段的独立可见性、表格对比与示例代码。
- [docs/ch05_05_use_binding.md](docs/ch05_05_use_binding.md)：使用 use 绑定模块成员英文文档，详细说明 use 的用法与示例代码。
- [docs/ch05_06_super_self.md](docs/ch05_06_super_self.md)：使用 super 与 self 简化路径英文文档，详细说明 super 与 self 的用法与示例代码。
- 欢迎反馈、补充和共建更多章节！
