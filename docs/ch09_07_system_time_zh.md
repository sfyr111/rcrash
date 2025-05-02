# 9.7 Rust 中的系统时间 SystemTime

本章讲解如何使用 `std::time::SystemTime` 获取系统时间、处理时间戳和计算时间间隔。

---

## 适用场景

- 获取当前系统时间（墙上时钟）
- 计算两个时间点之间的间隔
- 性能分析时测量耗时
- 与 UNIX 时间戳互转

---

## 示例代码

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn main() {
    // 1. 获取当前系统时间
    let now = SystemTime::now();
    println!("当前系统时间: {:?}", now);

    // 2. 转换为 UNIX 时间戳（自 1970-01-01 00:00:00 UTC 起的秒数）
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!("UNIX 时间戳: {} 秒", duration.as_secs());
            println!("UNIX 时间戳: {} 毫秒", duration.as_millis());
        },
        Err(e) => println!("SystemTime 在 UNIX EPOCH 之前! 错误: {:?}", e),
    }

    // 3. 时间加减 Duration
    let five_secs = Duration::new(5, 0);
    let later = now + five_secs;
    let earlier = now - five_secs;
    println!("5 秒后: {:?}", later);
    println!("5 秒前: {:?}", earlier);

    // 4. 测量耗时
    let start = SystemTime::now();
    // 模拟耗时操作
    std::thread::sleep(Duration::from_millis(100));
    let elapsed = start.elapsed().expect("时间倒流");
    println!("耗时: {} 毫秒", elapsed.as_millis());

    // 5. 由 UNIX 时间戳构造 SystemTime
    let unix_ts = 1_000_000_000u64; // 秒
    let t = UNIX_EPOCH + Duration::from_secs(unix_ts);
    println!("UNIX 时间戳 {} 对应的 SystemTime: {:?}", unix_ts, t);
}
```

## 常用 SystemTime 操作

- `SystemTime::now()` —— 获取当前时间
- `duration_since(UNIX_EPOCH)` —— 转为 UNIX 时间戳
- `SystemTime + Duration`、`SystemTime - Duration` —— 时间加减
- `elapsed()` —— 测量某一时刻到现在的耗时
- 由 UNIX 时间戳构造：`UNIX_EPOCH + Duration::from_secs(...)`

---

## 要点速览

- `SystemTime` 表示墙上时钟时间，非单调计时（计时用 `Instant`）
- 所有时间均为 UTC
- 注意处理可能的错误（系统时钟可能倒退）
- 若需格式化/解析时间，建议用 `chrono` 或 `time` 第三方库

---

运行代码：

```sh
cargo run --bin ch09_07_system_time
```
