# 9.3 Rust 中的引用计数 Rc<T>

本章详细讲解引用计数智能指针 `Rc<T>` 的用途、原理与注意事项。

---

## 什么时候用 Rc<T>？

Rc<T> 最常见的三大场景：

1. **需要让程序的多个部分共享某些数据的所有权**  
   *例：图结构、带共享节点的树结构。*
   - Rust 默认所有权模型要求“唯一所有者”。Rc<T> 通过引用计数支持多个所有者。

2. **编译期所有权规则过于严格，不适合你的数据结构**  
   *例：需要多个变量指向同一节点（非单一父节点）。*
   - Rc<T> 允许多个变量持有同一份数据，只有所有 Rc 都失效时数据才会被释放。

3. **Rc<T> 仅适用于单线程场景**  
   - 多线程场景请用 `Arc<T>`。

---

## 示例代码

```rust
use std::rc::Rc;

fn main() {
    // 示例1：简单的共享所有权
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
    println!("克隆后引用计数: {}", Rc::strong_count(&a));

    // 示例2：链表节点共享
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let tail = Rc::new(Cons(10, Rc::new(Nil)));
    let list1 = Cons(5, Rc::clone(&tail));
    let list2 = Cons(3, Rc::clone(&tail));
    println!("list1 = {:?}\nlist2 = {:?}", list1, list2);
    println!("尾节点引用计数: {}", Rc::strong_count(&tail));

    // Rc<T> 不提供内部可变性，也不保证线程安全。
    // 需要可变性可结合 RefCell<T>，多线程请用 Arc<T>。
}
```

## 要点速览

- `Rc<T>` 是一种引用计数智能指针，用于单线程下的共享所有权。
- 典型用途：
    1. 共享所有权（图、树等结构）
    2. 规避唯一所有者限制
    3. 不支持可变性和多线程
- Rc<T> 只允许不可变访问，若需可变性可与 `RefCell<T>` 结合。
- 多线程场景请用 `Arc<T>`。

---

运行代码：

```sh
cargo run --bin ch09_03_rc
```
