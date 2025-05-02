# 9.4 Rust 中的动态数组 Vec<T>

本章详细讲解 Rust 动态数组类型 `Vec<T>` 的用法、特性与最佳实践。

---

## 什么时候用 Vec<T>？

Vec<T> 是 Rust 最常用的可增长集合类型。适用场景：

1. **需要元素数量可变的集合**  
   - 集合大小在编译期未知，或运行时会动态变化。
2. **需要高效的随机访问和末尾插入/删除**  
   - Vec<T> 支持 O(1) 下标访问、O(1) 末尾 push/pop。
3. **需要收集迭代器结果或在运行时动态构建数据**  
   - 很多 API/迭代器都返回或接受 Vec<T>。

---

## 示例代码

```rust
fn main() {
    // 1. 创建空 Vec 并添加元素
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);

    // 2. 用初始值创建 Vec
    let mut v2 = vec![10, 20, 30];
    println!("v2 = {:?}", v2);

    // 3. 下标访问元素
    println!("第一个元素: {}", v[0]);

    // 4. get 方法安全访问
    match v.get(10) {
        Some(val) => println!("第 10 个元素: {}", val),
        None => println!("下标 10 没有元素"),
    }

    // 5. 遍历 Vec
    for x in &v {
        println!("element = {}", x);
    }

    // 6. 移除末尾元素
    v.pop();
    println!("pop 后: {:?}", v);

    // 7. 插入和移除
    v2.insert(1, 99); // 在下标 1 插入 99
    println!("插入后: {:?}", v2);
    v2.remove(2); // 移除下标 2 的元素
    println!("移除后: {:?}", v2);

    // 8. 其他常用方法
    println!("长度: {}, 是否为空: {}", v2.len(), v2.is_empty());
    v2.clear();
    println!("clear 后: {:?}, len = {}", v2, v2.len());

    // 9. 去重、排序、反转
    let mut v3 = vec![3, 1, 2, 2, 3, 1];
    v3.sort();
    v3.dedup();
    v3.reverse();
    println!("排序、去重、反转后: {:?}", v3);

    // 10. 保留、扩展、追加
    let mut v4 = vec![1, 2, 3, 4, 5];
    v4.retain(|&x| x % 2 == 1); // 保留奇数
    println!("保留奇数后: {:?}", v4);
    v4.extend(&[7, 9]);
    println!("extend 后: {:?}", v4);
    let mut v5 = vec![100, 200];
    v4.append(&mut v5);
    println!("append 后: {:?}, v5: {:?}", v4, v5);

    // 11. 迭代器 iter, iter_mut, into_iter
    let v6 = vec![10, 20, 30];
    for x in v6.iter() {
        print!("{} ", x);
    }
    println!("<- iter");
    let mut v7 = vec![1, 2, 3];
    for x in v7.iter_mut() {
        *x *= 2;
    }
    println!("iter_mut 后: {:?}", v7);
    for x in v7.into_iter() {
        print!("{} ", x);
    }
    println!("<- into_iter");

    // 12. 容量与扩容
    let mut v8 = Vec::with_capacity(10);
    println!("初始容量: {}", v8.capacity());
    v8.extend(0..5);
    println!("extend 后容量: {}, v8 = {:?}", v8.capacity(), v8);
    v8.reserve(20);
    println!("reserve(20) 后容量: {}", v8.capacity());
}
```

## 常用 Vec<T> 方法

- `push`, `pop`, `insert`, `remove`, `clear`, `len`, `is_empty`
- `contains`, `dedup`, `sort`, `reverse`, `retain`, `extend`, `append`
- `iter`, `iter_mut`, `into_iter`, `drain`, `split_at`, `split_off`
- `first`, `last`, `get`, `get_mut`, `resize`, `truncate`, `capacity`, `reserve`

上面代码展示了这些方法在实际业务场景中的常见用法。

## 要点速览

- `Vec<T>` 是可增长、堆分配的数组。
- 典型用途：
    1. 元素数量可变的集合
    2. 高效的末尾操作与随机访问
    3. 收集迭代器结果
- `[]` 下标越界会 panic，`get` 方法返回 Option 更安全。
- 遍历 Vec 简洁高效。
- 末尾 pop 操作是 O(1)。

---

运行代码：

```sh
cargo run --bin ch09_04_vec
