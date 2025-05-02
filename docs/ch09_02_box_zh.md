# 9.2 Rust 中的智能指针 Box<T>

本章详细讲解 Rust 智能指针 `Box<T>` 的核心用途、典型场景与原理。

---

## 什么时候用 Box<T>？

Box<T> 最常见的三大用法：

1. **编译时大小未知的类型，需要在已知大小的上下文中使用**  
   *例：递归类型（链表、树）、trait 对象（dyn Trait）。*
   - Rust 要求每个变量在编译时大小确定。递归类型或 trait 对象本身大小不定。
   - Box<T> 就像“快递盒子”，指针大小固定，把未知大小的东西装进堆里，编译器就能处理。

2. **大量数据，转移所有权且避免拷贝**  
   *例：大数组、大字符串等。*
   - Box<T> 把数据放堆上，转移所有权时只移动指针（8字节），数据本体不动，避免昂贵的数据拷贝。

3. **只关心是否实现了某个 trait，而不关心具体类型**  
   *例：trait 对象的动态分发，类似 Go 的 interface。*
   - Box<dyn Trait> 支持动态分发，只关心接口（trait），不关心具体类型。

---

## 示例代码

```rust
fn main() {
    // 1. 在堆上存储数据
    let b = Box::new(5);
    println!("b = {}", b);
    // 适合大数据或需要显式堆分配的场景。

    // 2. 递归类型（如链表）
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    // 不用 Box，编译器无法确定 List 的大小。

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    // 3. trait 对象：动态分发（见 9.3）
    trait Animal { fn speak(&self); }
    struct Dog;
    impl Animal for Dog { fn speak(&self) { println!("Woof"); } }
    let a: Box<dyn Animal> = Box::new(Dog);
    a.speak();
}
```

## 要点速览

- `Box<T>` 是一种智能指针，用于在堆上分配值。
- 典型用途：
    1. 递归/动态大小类型
    2. 大数据所有权转移且避免拷贝
    3. trait 对象动态分发
- 解引用 `Box<T>` 可访问内部值。
- Box 拥有的值会在超出作用域时自动释放。

---

运行代码：

```sh
cargo run --bin ch09_02_box
