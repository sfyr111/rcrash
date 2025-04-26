# Chapter 3.10: Struct

## Key Points

- Structs are custom data types that let you name and package together multiple related values.
- 三种形式：普通结构体、元组结构体、单元结构体。
- 用 `#[derive(Debug)]` 可以用 `{:?}` 一次性打印整个结构体。
- 关联函数（如 `fn new`）常用作构造器，不带 self 参数，用 `User::new(...)` 调用。
- 关联方法（如 `fn show(&self)`) 作用于实例，用 `user.show()` 调用。
- 用 `..` 语法可以从另一个实例快速创建新结构体。
- **注意：如果元组结构体字段没被单独访问，会有 dead_code 警告。建议像 `println!("Color fields: {}, {}, {}", black.0, black.1, black.2);` 这样显式访问字段。**

## Demo Code

```rust
fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
    }

    impl User {
        fn new(username: &str, email: &str) -> User {
            User {
                username: username.to_string(),
                email: email.to_string(),
                sign_in_count: 1,
                active: true,
            }
        }
        fn show(&self) {
            println!("User info: username={}, email={}, sign_in_count={}, active={}",
                self.username, self.email, self.sign_in_count, self.active);
        }
    }

    let mut user1 = User::new("alice", "alice@example.com");
    println!("user1 = {:?}", user1); // 一次性打印
    user1.show(); // 用方法打印

    // 字段访问和修改
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign in count: {}", user1.sign_in_count);
    user1.email = String::from("alice@newmail.com");
    println!("Updated email: {}", user1.email);
    user1.show();

    // 结构体更新语法
    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("user2 = {:?}", user2);
    user2.show();

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = {:?}", black);
    // 使用字段，避免 dead_code 警告
    println!("Color fields: {}, {}, {}", black.0, black.1, black.2);

    // 单元结构体
    #[derive(Debug)]
    struct Marker;
    let m = Marker;
    println!("marker = {:?}", m);
}
```

## Table: Struct Features

| Feature                 | Example                                      | Notes                                   |
|-------------------------|----------------------------------------------|-----------------------------------------|
| Define struct           | `struct User { ... }`                        | Named fields                            |
| Create instance         | `let u = User::new(...)`                     | 推荐用 new 构造器                       |
| Associated function     | `impl User { fn new(...) -> User { ... } }`  | 构造器/辅助函数 via `impl`              |
| Associated method       | `impl User { fn show(&self) { ... } }`       | 用于实例，`u.show()`                     |
| Print whole struct      | `println!("{:?}", u)`                        | 需 `#[derive(Debug)]`                    |
| Access field            | `u.username`                                 | Use dot syntax                          |
| Mutable instance        | `let mut u = User { ... }`                   | Allows modifying fields                 |
| Update field            | `u.email = ...`                              | Only if instance is mutable             |
| Struct update syntax    | `User { ..u1 }`                              | Copy remaining fields from another      |
| Tuple struct            | `struct Color(i32, i32, i32);`               | Fields have no names                    |
| Tuple struct access     | `c.0, c.1, c.2`                              | Access by index，避免 dead_code 警告      |
| Unit-like struct        | `struct Marker;`                             | No fields, just a marker                |
| Unit-like instance      | `let m = Marker;`                            | Used for marker types                   |

## Notes

- 用 `#[derive(Debug)]` + `{:?}` 可以方便调试和一次性打印结构体。
- 关联函数常用于构造实例，关联方法可对实例做操作。
- 元组结构体字段如果没被单独访问，会有 dead_code 警告，建议显式访问。
- 字段名让代码更易读，减少出错。
- 元组结构体适合字段无语义时，单元结构体常用于类型标记或 trait。

---

> For more details, see [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
