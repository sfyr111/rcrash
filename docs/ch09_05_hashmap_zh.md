# 9.5 Rust 中的 HashMap（哈希映射）

本章详细讲解 Rust 哈希映射类型 `HashMap<K, V>` 的用法、特性与最佳实践。

---

## 什么时候用 HashMap？

HashMap 是 Rust 最常用的关联型集合。适用场景：

1. **需要通过键高效查找、插入、删除值**  
   - HashMap 支持平均 O(1) 的查找、插入和删除。
2. **需要按键对数据分组、计数或索引**  
   - 适合用于频率统计、分组聚合、索引。
3. **需要高效判断键是否存在或按键更新值**  
   - entry API 支持条件插入和原位修改。

---

## 示例代码

```rust
use std::collections::HashMap;

fn main() {
    // 1. 创建空 HashMap 并插入键值对
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("scores = {:?}", scores);

    // 2. 按键访问值
    let team_name = "Blue";
    match scores.get(team_name) {
        Some(score) => println!("{} 队得分: {}", team_name, score),
        None => println!("没有 {} 队得分", team_name),
    }

    // 3. 只读遍历所有键值对（&hashmap）
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 3.1 可变遍历所有键值对（&mut hashmap）
    for (key, value) in &mut scores {
        println!("{}: {} (mutable)", key, value);
    }

    // 3.2 只遍历所有 key
    for key in scores.keys() {
        println!("key: {}", key);
    }

    // 3.3 只遍历所有 value
    for value in scores.values() {
        println!("value: {}", value);
    }

    // 3.4 消费性遍历（into_iter，所有权转移）
    let scores_clone = scores.clone();
    for (key, value) in scores_clone.into_iter() {
        println!("{}: {} (owned)", key, value);
    }

    // 3.5 带索引遍历（enumerate）
    for (i, (key, value)) in scores.iter().enumerate() {
        println!("#{}: {} => {}", i, key, value);
    }

    // 4. 更新某个键的值
    scores.insert("Blue", 25);
    println!("更新后: {:?}", scores);

    // 5. 仅在键不存在时插入（entry API）
    scores.entry("Green").or_insert(30);
    println!("entry or_insert 后: {:?}", scores);

    // 6. 原位修改（entry API）
    scores.entry("Blue").and_modify(|v| *v += 10);
    println!("entry and_modify 后: {:?}", scores);

    // 7. 移除键
    scores.remove("Yellow");
    println!("移除后: {:?}", scores);

    // 8. 判断键是否存在
    println!("包含 'Blue'? {}", scores.contains_key("Blue"));
    println!("包含 'Yellow'? {}", scores.contains_key("Yellow"));

    // 9. 其他常用方法
    println!("长度: {}, 是否为空: {}", scores.len(), scores.is_empty());
    scores.clear();
    println!("clear 后: {:?}", scores);
}
```

## 常用 HashMap 方法

- `insert`, `get`, `get_mut`, `remove`, `contains_key`, `entry`, `keys`, `values`, `iter`, `iter_mut`
- `len`, `is_empty`, `clear`, `drain`, `extend`, `retain`

上面代码展示了这些方法在实际业务场景中的常见用法。

---

## 要点速览

- `HashMap<K, V>` 是基于哈希表实现的可增长键值存储。
- 典型用途：
    1. 快速按键查找与更新
    2. 按键分组、计数、索引
    3. 高效判断键存在与条件插入/修改
- 键类型需实现 `Eq` 和 `Hash` trait。
- entry API 支持灵活的插入或更新逻辑。
- 遍历顺序不保证。

---

运行代码：

```sh
cargo run --bin ch09_05_hashmap
