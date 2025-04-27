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
```

---

## 章节索引

| 章节   | 主题           | 示例代码                                                        | 配套文档                                                      |
|--------|----------------|-----------------------------------------------------------------|---------------------------------------------------------------|
| 3.3    | 变量与可变性   | [ch03_3_var_mut.rs](src/bin/ch03_3_var_mut.rs)                  | [ch03_3_var_mut.md](docs/ch03_3_var_mut.md)                   |
| 3.4    | 基本数据类型   | [ch03_4_basic_types.rs](src/bin/ch03_4_basic_types.rs)          | [ch03_4_basic_types.md](docs/ch03_4_basic_types.md)            |
| 3.6    | 整数溢出       | [ch03_6_integer_overflow.rs](src/bin/ch03_6_integer_overflow.rs)| [ch03_6_integer_overflow.md](docs/ch03_6_integer_overflow.md)  |
| 3.7    | 元组           | [ch03_7_tuple.rs](src/bin/ch03_7_tuple.rs)                      | [ch03_7_tuple.md](docs/ch03_7_tuple.md)                        |
| 3.8    | 数组           | [ch03_8_array.rs](src/bin/ch03_8_array.rs)                      | [ch03_8_array.md](docs/ch03_8_array.md)                        |
| 3.9    | 切片类型       | [ch03_9_slice.rs](src/bin/ch03_9_slice.rs)                      | [ch03_9_slice.md](docs/ch03_9_slice.md)                        |
| 3.10   | 结构体         | [ch03_10_struct.rs](src/bin/ch03_10_struct.rs)                  | [ch03_10_struct.md](docs/ch03_10_struct.md)                    |
| 3.11   | 枚举           | [ch03_11_enum.rs](src/bin/ch03_11_enum.rs)                      | [ch03_11_enum.md](docs/ch03_11_enum.md)                        |
| 3.12   | 注释类型       | [ch03_12_comment.rs](src/bin/ch03_12_comment.rs)                | [ch03_12_comment.md](docs/ch03_12_comment.md)                  |
| 3.13   | println! 宏    | [ch03_13_println.rs](src/bin/ch03_13_println.rs)                | [ch03_13_println.md](docs/ch03_13_println.md)                  |
| 3.14   | 类型转换       | [ch03_14_cast.rs](src/bin/ch03_14_cast.rs)                      | [ch03_14_cast.md](docs/ch03_14_cast.md)                        |
| 4.2    | 表达式形式     | [ch04_2_expr.rs](src/bin/ch04_2_expr.rs)                        | [ch04_2_expr.md](docs/ch04_2_expr.md)                          |
| 4.3    | if/else 选择结构| [ch04_3_if.rs](src/bin/ch04_3_if.rs)                            | [ch04_3_if.md](docs/ch04_3_if.md)                              |
| 4.4    | loop 循环      | [ch04_4_loop.rs](src/bin/ch04_4_loop.rs)                        | [ch04_4_loop.md](docs/ch04_4_loop.md)                          |
| 4.5    | while 循环     | [ch04_5_while.rs](src/bin/ch04_5_while.rs)                      | [ch04_5_while.md](docs/ch04_5_while.md)                        |
| 4.6    | for/range 迭代 | [ch04_6_for.rs](src/bin/ch04_6_for.rs)                          | [ch04_6_for.md](docs/ch04_6_for.md)                            |

> 更多章节持续更新中...

---

## 说明

- 每一章的代码和文档都双向链接，便于快速查阅。
- 推荐结合 [Rust 官方书籍](https://kaisery.github.io/trpl-zh-cn/) 学习。
- 枚举的打印全部为手动 match，无自动 Debug。
- 欢迎反馈、补充和共建更多章节！
