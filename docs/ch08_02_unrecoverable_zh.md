# 8.2 不可恢复的错误（Unrecoverable Errors）

本章演示 Rust 如何通过 `panic!` 宏和相关模式处理不可恢复的错误。

## 示例代码

```rust
fn main() {
    // 使用 panic! 显式触发不可恢复错误
    // panic!("This is an unrecoverable error!");

    // 越界访问会导致 panic
    let v = vec![1, 2, 3];
    // let out_of_bounds = v[99]; // 取消注释可观察 panic

    // 断言失败
    // assert!(1 == 2, "断言失败：1 != 2");

    // 尚未实现
    // unimplemented!("此部分尚未实现");

    // 不可达代码
    // unreachable!("此代码不应被执行");

    println!("Program completed without panic.");

    // 函数中的 panic 示例
    // let result = always_fails();
}

// 总是 panic 的函数
fn always_fails() {
    panic!("This function always panics!");
}

// 查看完整 panic 堆栈信息，请设置环境变量：
// RUST_BACKTRACE=1
// 例如（终端执行）：
// $ RUST_BACKTRACE=1 cargo run --bin ch08_02_unrecoverable
```

## 要点速览

- 使用 `panic!` 显式触发不可恢复错误。
- 使用 `assert!` 和 `assert_eq!` 进行运行时断言，失败会导致 panic。
- 使用 `unimplemented!` 标记尚未实现的代码。
- 使用 `unreachable!` 标记不可达的代码路径。
- 常见触发场景：手动 panic、越界访问、断言失败等。
- panic 时当前线程会展开栈并清理资源（默认行为）。
- 若需查看完整堆栈信息，运行前设置环境变量 `RUST_BACKTRACE=1`。
- 不可恢复错误用于 bug 或无法安全继续的场景。
- 可恢复错误请使用 `Result` 类型（见后续章节）。

## 不可恢复错误常见场景表

| 场景                   | 示例代码                          |
|------------------------|-----------------------------------|
| 手动 panic             | `panic!("error message")`         |
| 向量越界访问           | `let x = v[99];`                  |
| 断言失败               | `assert!(1 == 2);`、`assert_eq!(a, b);`            |
| 尚未实现               | `unimplemented!("not done")`                      |
| 不可达代码             | `unreachable!("should not happen")`                |

---

运行代码：

```sh
cargo run --bin ch08_02_unrecoverable
